package history

import (
	"encoding/json"
	"fmt"
	"net/http"
	"net/url"
	"strconv"
	"strings"
	"sync"
	"time"

	"github.com/adshao/go-binance"
	"github.com/adshao/go-binance/common"
	"github.com/bitly/go-simplejson"
	"github.com/hiroaki-yamamoto/midas/backend/services/testbeds/models"
	"github.com/schollz/progressbar/v3"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
	"go.uber.org/zap"
	"golang.org/x/net/context"
)

const numConcReq = 6

type klinesError struct {
	Klines       []*models.Kline
	Err          error
	IncrementObj int64
}

type bulkFetchRequest struct {
	Pair  string
	Start time.Time
	End   time.Time
}

// Binance represents binance historical chart data downloader.
type Binance struct {
	Logger    *zap.Logger
	Col       *mongo.Collection
	Cli       *binance.Client
	wg        *sync.WaitGroup
	ctx       context.Context
	cancelCtx context.CancelFunc
}

// NewBinance constructs a new instance of Binance.
func NewBinance(log *zap.Logger, col *mongo.Collection) *Binance {
	ctx, cancel := context.WithCancel(context.Background())
	return &Binance{
		Logger:    log,
		Col:       col,
		Cli:       binance.NewClient("", ""),
		wg:        &sync.WaitGroup{},
		ctx:       ctx,
		cancelCtx: cancel,
	}
}

// Stop stops the current running tasks.
func (me *Binance) Stop() {
	me.wg.Wait()
	me.cancelCtx()
}

func (me *Binance) fetch(
	pair string,
	startAt, endAt int64,
) ([]*models.Kline, error) {
	const endpoint = "https://api.binance.com/api/v3/klines"
	query := make(url.Values)
	query.Set("symbol", pair)
	query.Set("interval", "1m")
	query.Set("startTime", strconv.FormatInt(startAt*1000, 10))
	if endAt > 0 {
		query.Set("endTime", strconv.FormatInt(endAt*1000, 10))
	}
	timeDiff := int64(endAt-startAt) / 60
	if timeDiff > 0 {
		query.Set("limit", strconv.FormatInt(timeDiff, 10))
	} else {
		query.Set("limit", "1")
	}
	url := fmt.Sprintf("%s?%s", endpoint, query.Encode())
	for {
		resp, err := http.Get(url)
		if err != nil {
			return nil, err
		}
		switch resp.StatusCode {
		case http.StatusOK:
			j, err := simplejson.NewFromReader(resp.Body)
			if err != nil {
				return nil, err
			}
			jLen := len(j.MustArray())
			klines := make([]*models.Kline, jLen)
			for ind := 0; ind < jLen; ind++ {
				item := j.GetIndex(ind)
				var open, close, high, low, vol float64
				var QuoteAssetVolume, TakerBuyBaseAssetVolume, TakerBuyQuoteAssetVolume float64
				var err error
				if open, err = strconv.ParseFloat(item.GetIndex(1).MustString(), 64); err != nil {
					return nil, err
				}
				if close, err = strconv.ParseFloat(item.GetIndex(4).MustString(), 64); err != nil {
					return nil, err
				}
				if high, err = strconv.ParseFloat(item.GetIndex(2).MustString(), 64); err != nil {
					return nil, err
				}
				if low, err = strconv.ParseFloat(item.GetIndex(3).MustString(), 64); err != nil {
					return nil, err
				}
				if vol, err = strconv.ParseFloat(item.GetIndex(5).MustString(), 64); err != nil {
					return nil, err
				}
				if QuoteAssetVolume, err = strconv.ParseFloat(
					item.GetIndex(7).MustString(), 64,
				); err != nil {
					return nil, err
				}
				if TakerBuyBaseAssetVolume, err = strconv.ParseFloat(
					item.GetIndex(9).MustString(), 64,
				); err != nil {
					return nil, err
				}
				if TakerBuyQuoteAssetVolume, err = strconv.ParseFloat(
					item.GetIndex(10).MustString(), 64,
				); err != nil {
					return nil, err
				}
				klines[ind] = &models.Kline{
					Symbol: pair,
					OpenAt: time.Unix(
						item.GetIndex(0).MustInt64()/1000,
						int64(
							time.Duration(item.GetIndex(0).MustInt64()%1000)*time.Millisecond,
						),
					),
					Open:   open,
					High:   high,
					Low:    low,
					Close:  close,
					Volume: vol,
					CloseAt: time.Unix(
						item.GetIndex(6).MustInt64()/1000,
						int64(
							time.Duration(item.GetIndex(6).MustInt64()%1000)*time.Millisecond,
						),
					),
					QuoteAssetVolume:         QuoteAssetVolume,
					TradeNum:                 item.GetIndex(8).MustInt64(),
					TakerBuyBaseAssetVolume:  TakerBuyBaseAssetVolume,
					TakerBuyQuoteAssetVolume: TakerBuyQuoteAssetVolume,
				}
			}
			return klines, nil
		case 418, 429:
			waitCount, err := strconv.ParseUint(
				resp.Header.Get("Retry-After"), 10, 64,
			)
			if err != nil || waitCount < 20 {
				err = nil
				waitCount = 20
			}
			me.Logger.Warn(
				"Waiting...",
				zap.Int("status", resp.StatusCode),
				zap.Uint64("duration", waitCount),
			)
			<-time.After(time.Duration(waitCount) * time.Second)
			break
		case http.StatusNotFound:
			me.Logger.Warn("The response status code got NotFound...")
			return nil, nil
		default:
			me.Logger.Warn(
				"Got irregular status code.",
				zap.String("URL", url),
				zap.Int("code", resp.StatusCode),
			)
			dec := json.NewDecoder(resp.Body)
			apiErr := &common.APIError{}
			if decErr := dec.Decode(apiErr); decErr != nil {
				return nil, decErr
			}
			return nil, apiErr
		}
	}
}

func (me *Binance) bulkFetch(
	times <-chan *bulkFetchRequest,
	results chan<- *klinesError,
) {
	for t := range times {
		res, err := me.fetch(t.Pair, t.Start.Unix(), t.End.Unix())
		results <- &klinesError{
			Klines:       res,
			Err:          err,
			IncrementObj: int64(t.End.Sub(t.Start) / time.Minute),
		}
	}
}

func (me *Binance) record(results <-chan *klinesError, done chan<- int64) {
	for res := range results {
		func(res *klinesError) {
			if res.Err != nil {
				me.Logger.Warn("Error while fetching", zap.Error(res.Err))
				return
			}
			if res.Klines == nil || len(res.Klines) < 1 {
				return
			}
			me.wg.Add(1)
			go func(klines []*models.Kline) {
				defer me.wg.Done()
				toInsert := make([]interface{}, len(klines))
				for ind, item := range klines {
					toInsert[ind] = item
				}
				dbCtx, stop := context.WithTimeout(me.ctx, 30*time.Second)
				defer stop()
				if _, err := me.Col.InsertMany(dbCtx, toInsert); err != nil {
					me.Logger.Warn(
						"Error while inseting data to the db",
						zap.Error(err),
						zap.String("pair", klines[0].Symbol),
					)
				}
			}(res.Klines)
		}(res)
		done <- res.IncrementObj
		select {
		case <-me.ctx.Done():
			return
		default:
			break
		}
	}
}

// Run starts downloading Historical data.
func (me *Binance) Run(pair string) error {
	info, err := me.Cli.NewExchangeInfoService().Do(me.ctx)
	if err != nil {
		return err
	}
	var targetSymbols []string
	if pair == "all" {
		for _, sym := range info.Symbols {
			if strings.ToUpper(sym.Status) == "TRADING" {
				targetSymbols = append(targetSymbols, sym.Symbol)
			}
		}
	} else {
		for _, sym := range info.Symbols {
			if strings.ToUpper(sym.Symbol) == strings.ToUpper(pair) &&
				strings.ToUpper(sym.Status) == "TRADING" {
				targetSymbols = append(targetSymbols, sym.Symbol)
				break
			}
		}
	}
	if len(targetSymbols) < 1 {
		return &NoSuchPair{
			Pair: pair,
		}
	}
	me.Logger.Info("Number of pairs to fetch", zap.Any("numPairs", len(targetSymbols)))
	endAt := time.Now().UTC()
	endAt = endAt.Add(
		-time.Duration(endAt.Second())*time.Second -
			time.Duration(endAt.Nanosecond())*time.Nanosecond,
	)

	for ind, pair := range targetSymbols {
		firstKlines, err := me.fetch(pair, 0, 0)
		if err != nil {
			me.Logger.Error("Error on fetching first kline date", zap.Error(err))
		}
		firstKline := firstKlines[0]
		cacheCtx, cancelFind := context.WithTimeout(me.ctx, 10*time.Second)
		defer cancelFind()
		if cur, err := me.Col.Find(
			cacheCtx,
			bson.M{"symbol": pair},
			options.Find().SetSort(bson.M{"closeat": -1}).SetLimit(1)); err == nil {
			if cur.Next(cacheCtx) {
				kline := &models.Kline{}
				cur.Decode(kline)
				firstKline = kline
			}
		}

		numObj := int64(endAt.Sub(firstKline.CloseAt) / time.Minute)
		startAt := endAt.Add(time.Duration(-numObj) * time.Minute)
		if numObj > 1000 {
			startAt = endAt.Add(-1000 * time.Minute)
		}
		recentEndAt := endAt
		recentStartAt := startAt
		bar := progressbar.Default(int64(numObj))
		bar.Describe(fmt.Sprintf("%s [%d/%d]", pair, ind+1, len(targetSymbols)))
		bar.RenderBlank()

		chanBuffCap := numObj / 1000
		if numObj%1000 > 0 {
			chanBuffCap++
		}

		func() {
			fetchReq := make(chan *bulkFetchRequest, chanBuffCap)
			results := make(chan *klinesError, chanBuffCap)
			done := make(chan int64, chanBuffCap)
			defer func() {
				close(fetchReq)
				close(results)
				close(done)
			}()

			for i := 0; i < numConcReq; i++ {
				go me.bulkFetch(fetchReq, results)
				go me.record(results, done)
			}

			for i := int64(0); i < chanBuffCap; i++ {
				func() {
					defer func() {
						endAt = startAt
						startdiff := startAt.Sub(firstKline.OpenAt) / time.Minute
						if startdiff > 1000 {
							startdiff = 1000
						}
						startAt = endAt.Add(-startdiff * time.Minute)
					}()
					fetchReq <- &bulkFetchRequest{
						Pair:  pair,
						Start: startAt,
						End:   endAt,
					}
				}()
				select {
				case <-me.ctx.Done():
					return
				default:
					continue
				}
			}
			for i := 0; i < cap(done); i++ {
				inc := <-done
				bar.Add64(inc)
			}
			bar.Finish()
		}()
		startAt, endAt = recentStartAt, recentEndAt
	}
	return nil
}