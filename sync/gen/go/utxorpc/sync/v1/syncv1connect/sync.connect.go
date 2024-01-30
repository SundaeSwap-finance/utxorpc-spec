// Code generated by protoc-gen-connect-go. DO NOT EDIT.
//
// Source: utxorpc/sync/v1/sync.proto

package syncv1connect

import (
	context "context"
	errors "errors"
	connect_go "github.com/bufbuild/connect-go"
	v1 "github.com/utxorpc/spec/sync/gen/go/utxorpc/sync/v1"
	http "net/http"
	strings "strings"
)

// This is a compile-time assertion to ensure that this generated file and the connect package are
// compatible. If you get a compiler error that this constant is not defined, this code was
// generated with a version of connect newer than the one compiled into your binary. You can fix the
// problem by either regenerating this code with an older version of connect or updating the connect
// version compiled into your binary.
const _ = connect_go.IsAtLeastVersion0_1_0

const (
	// ChainSyncServiceName is the fully-qualified name of the ChainSyncService service.
	ChainSyncServiceName = "utxorpc.sync.v1.ChainSyncService"
)

// These constants are the fully-qualified names of the RPCs defined in this package. They're
// exposed at runtime as Spec.Procedure and as the final two segments of the HTTP route.
//
// Note that these are different from the fully-qualified method names used by
// google.golang.org/protobuf/reflect/protoreflect. To convert from these constants to
// reflection-formatted method names, remove the leading slash and convert the remaining slash to a
// period.
const (
	// ChainSyncServiceFetchBlockProcedure is the fully-qualified name of the ChainSyncService's
	// FetchBlock RPC.
	ChainSyncServiceFetchBlockProcedure = "/utxorpc.sync.v1.ChainSyncService/FetchBlock"
	// ChainSyncServiceDumpHistoryProcedure is the fully-qualified name of the ChainSyncService's
	// DumpHistory RPC.
	ChainSyncServiceDumpHistoryProcedure = "/utxorpc.sync.v1.ChainSyncService/DumpHistory"
	// ChainSyncServiceFollowTipProcedure is the fully-qualified name of the ChainSyncService's
	// FollowTip RPC.
	ChainSyncServiceFollowTipProcedure = "/utxorpc.sync.v1.ChainSyncService/FollowTip"
)

// ChainSyncServiceClient is a client for the utxorpc.sync.v1.ChainSyncService service.
type ChainSyncServiceClient interface {
	FetchBlock(context.Context, *connect_go.Request[v1.FetchBlockRequest]) (*connect_go.Response[v1.FetchBlockResponse], error)
	DumpHistory(context.Context, *connect_go.Request[v1.DumpHistoryRequest]) (*connect_go.Response[v1.DumpHistoryResponse], error)
	FollowTip(context.Context, *connect_go.Request[v1.FollowTipRequest]) (*connect_go.ServerStreamForClient[v1.FollowTipResponse], error)
}

// NewChainSyncServiceClient constructs a client for the utxorpc.sync.v1.ChainSyncService service.
// By default, it uses the Connect protocol with the binary Protobuf Codec, asks for gzipped
// responses, and sends uncompressed requests. To use the gRPC or gRPC-Web protocols, supply the
// connect.WithGRPC() or connect.WithGRPCWeb() options.
//
// The URL supplied here should be the base URL for the Connect or gRPC server (for example,
// http://api.acme.com or https://acme.com/grpc).
func NewChainSyncServiceClient(httpClient connect_go.HTTPClient, baseURL string, opts ...connect_go.ClientOption) ChainSyncServiceClient {
	baseURL = strings.TrimRight(baseURL, "/")
	return &chainSyncServiceClient{
		fetchBlock: connect_go.NewClient[v1.FetchBlockRequest, v1.FetchBlockResponse](
			httpClient,
			baseURL+ChainSyncServiceFetchBlockProcedure,
			opts...,
		),
		dumpHistory: connect_go.NewClient[v1.DumpHistoryRequest, v1.DumpHistoryResponse](
			httpClient,
			baseURL+ChainSyncServiceDumpHistoryProcedure,
			opts...,
		),
		followTip: connect_go.NewClient[v1.FollowTipRequest, v1.FollowTipResponse](
			httpClient,
			baseURL+ChainSyncServiceFollowTipProcedure,
			opts...,
		),
	}
}

// chainSyncServiceClient implements ChainSyncServiceClient.
type chainSyncServiceClient struct {
	fetchBlock  *connect_go.Client[v1.FetchBlockRequest, v1.FetchBlockResponse]
	dumpHistory *connect_go.Client[v1.DumpHistoryRequest, v1.DumpHistoryResponse]
	followTip   *connect_go.Client[v1.FollowTipRequest, v1.FollowTipResponse]
}

// FetchBlock calls utxorpc.sync.v1.ChainSyncService.FetchBlock.
func (c *chainSyncServiceClient) FetchBlock(ctx context.Context, req *connect_go.Request[v1.FetchBlockRequest]) (*connect_go.Response[v1.FetchBlockResponse], error) {
	return c.fetchBlock.CallUnary(ctx, req)
}

// DumpHistory calls utxorpc.sync.v1.ChainSyncService.DumpHistory.
func (c *chainSyncServiceClient) DumpHistory(ctx context.Context, req *connect_go.Request[v1.DumpHistoryRequest]) (*connect_go.Response[v1.DumpHistoryResponse], error) {
	return c.dumpHistory.CallUnary(ctx, req)
}

// FollowTip calls utxorpc.sync.v1.ChainSyncService.FollowTip.
func (c *chainSyncServiceClient) FollowTip(ctx context.Context, req *connect_go.Request[v1.FollowTipRequest]) (*connect_go.ServerStreamForClient[v1.FollowTipResponse], error) {
	return c.followTip.CallServerStream(ctx, req)
}

// ChainSyncServiceHandler is an implementation of the utxorpc.sync.v1.ChainSyncService service.
type ChainSyncServiceHandler interface {
	FetchBlock(context.Context, *connect_go.Request[v1.FetchBlockRequest]) (*connect_go.Response[v1.FetchBlockResponse], error)
	DumpHistory(context.Context, *connect_go.Request[v1.DumpHistoryRequest]) (*connect_go.Response[v1.DumpHistoryResponse], error)
	FollowTip(context.Context, *connect_go.Request[v1.FollowTipRequest], *connect_go.ServerStream[v1.FollowTipResponse]) error
}

// NewChainSyncServiceHandler builds an HTTP handler from the service implementation. It returns the
// path on which to mount the handler and the handler itself.
//
// By default, handlers support the Connect, gRPC, and gRPC-Web protocols with the binary Protobuf
// and JSON codecs. They also support gzip compression.
func NewChainSyncServiceHandler(svc ChainSyncServiceHandler, opts ...connect_go.HandlerOption) (string, http.Handler) {
	chainSyncServiceFetchBlockHandler := connect_go.NewUnaryHandler(
		ChainSyncServiceFetchBlockProcedure,
		svc.FetchBlock,
		opts...,
	)
	chainSyncServiceDumpHistoryHandler := connect_go.NewUnaryHandler(
		ChainSyncServiceDumpHistoryProcedure,
		svc.DumpHistory,
		opts...,
	)
	chainSyncServiceFollowTipHandler := connect_go.NewServerStreamHandler(
		ChainSyncServiceFollowTipProcedure,
		svc.FollowTip,
		opts...,
	)
	return "/utxorpc.sync.v1.ChainSyncService/", http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		switch r.URL.Path {
		case ChainSyncServiceFetchBlockProcedure:
			chainSyncServiceFetchBlockHandler.ServeHTTP(w, r)
		case ChainSyncServiceDumpHistoryProcedure:
			chainSyncServiceDumpHistoryHandler.ServeHTTP(w, r)
		case ChainSyncServiceFollowTipProcedure:
			chainSyncServiceFollowTipHandler.ServeHTTP(w, r)
		default:
			http.NotFound(w, r)
		}
	})
}

// UnimplementedChainSyncServiceHandler returns CodeUnimplemented from all methods.
type UnimplementedChainSyncServiceHandler struct{}

func (UnimplementedChainSyncServiceHandler) FetchBlock(context.Context, *connect_go.Request[v1.FetchBlockRequest]) (*connect_go.Response[v1.FetchBlockResponse], error) {
	return nil, connect_go.NewError(connect_go.CodeUnimplemented, errors.New("utxorpc.sync.v1.ChainSyncService.FetchBlock is not implemented"))
}

func (UnimplementedChainSyncServiceHandler) DumpHistory(context.Context, *connect_go.Request[v1.DumpHistoryRequest]) (*connect_go.Response[v1.DumpHistoryResponse], error) {
	return nil, connect_go.NewError(connect_go.CodeUnimplemented, errors.New("utxorpc.sync.v1.ChainSyncService.DumpHistory is not implemented"))
}

func (UnimplementedChainSyncServiceHandler) FollowTip(context.Context, *connect_go.Request[v1.FollowTipRequest], *connect_go.ServerStream[v1.FollowTipResponse]) error {
	return connect_go.NewError(connect_go.CodeUnimplemented, errors.New("utxorpc.sync.v1.ChainSyncService.FollowTip is not implemented"))
}
