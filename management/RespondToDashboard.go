package management

import (
	"html/template"
	"net/http"

	"github.com/distantmagic/paddler/goroutine"
	"github.com/distantmagic/paddler/loadbalancer"
)

type RespondToDashboard struct {
	DashboardTemplates  *template.Template
	LoadBalancer        *loadbalancer.LoadBalancer
	ServerEventsChannel chan<- goroutine.ResultMessage
}

func (self *RespondToDashboard) ServeHTTP(response http.ResponseWriter, request *http.Request) {
	mutexToken := self.LoadBalancer.LoadBalancerTargetCollection.RBMutex.RLock()
	defer self.LoadBalancer.LoadBalancerTargetCollection.RBMutex.RUnlock(mutexToken)

	response.Header().Set("Content-Type", "text/html")
	response.WriteHeader(http.StatusOK)

	err := self.DashboardTemplates.ExecuteTemplate(response, "index.html", &RespondToDashboardTemplateProps{
		LlamaCppTargets:    self.LoadBalancer.LoadBalancerTargetCollection.Targets,
		LoadBalancerStatus: self.LoadBalancer.GetStatus(),
	})

	if err != nil {
		self.ServerEventsChannel <- goroutine.ResultMessage{
			Error: err,
		}

		return
	}
}
