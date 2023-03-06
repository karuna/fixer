use crate::internal::event::{Event, LOGOUT_TIMEOUT};
use crate::message::Message;
use crate::session::{
    in_session::InSession,
    latent_state::LatentState,
    session_state::{ConnectedNotLoggedOn, SessionState},
    Session,
};
use async_trait::async_trait;
use delegate::delegate;

#[derive(Default)]
pub struct LogoutState {
    connected_not_logged_on: ConnectedNotLoggedOn,
}

impl ToString for LogoutState {
    fn to_string(&self) -> String {
        String::from("Logout State")
    }
}

#[async_trait]
impl SessionState for LogoutState {
    delegate! {
        to self.connected_not_logged_on {
            fn is_connected(&self) -> bool;
            fn is_session_time(&self) -> bool;
            fn is_logged_on(&self) -> bool;
            fn shutdown_now(&self, _session: &Session);
        }
    }

    async fn fix_msg_in(self, session: &'_ mut Session, msg: &'_ Message) -> Box<dyn SessionState> {
        let next_state = InSession::default().fix_msg_in(session, msg);
        // 	nextState = inSession{}.FixMsgIn(session, msg)
        // 	if nextState, ok := nextState.(latentState); ok {
        // 		return nextState
        // 	}
        Box::new(self)
    }

    fn timeout(self, session: &mut Session, event: Event) -> Box<dyn SessionState> {
        if event == LOGOUT_TIMEOUT {
            session
                .log
                .on_event("Timed out waiting for logout response");
            return Box::new(LatentState::default());
        }

        Box::new(self)
    }

    fn stop(self, _session: &mut Session) -> Box<dyn SessionState> {
        Box::new(self)
    }
}

#[cfg(test)]
mod tests {
    // type LogoutStateTestSuite struct {
    //     SessionSuiteRig
    // }

    // func TestLogoutStateTestSuite(t *testing.T) {
    //     suite.Run(t, new(LogoutStateTestSuite))
    // }

    // func (s *LogoutStateTestSuite) SetupTest() {
    //     s.Init()
    //     s.session.State = logoutState{}
    // }

    // func (s *LogoutStateTestSuite) TestPreliminary() {
    //     s.False(s.session.IsLoggedOn())
    //     s.True(s.session.IsConnected())
    //     s.True(s.session.IsSessionTime())
    // }

    // func (s *LogoutStateTestSuite) TestTimeoutLogoutTimeout() {
    //     s.MockApp.On("OnLogout").Return(nil)
    //     s.Timeout(s.session, internal.LogoutTimeout)

    //     s.MockApp.AssertExpectations(s.T())
    //     s.State(latentState{})
    // }

    // func (s *LogoutStateTestSuite) TestTimeoutNotLogoutTimeout() {
    //     tests := []internal.Event{internal.PeerTimeout, internal.NeedHeartbeat, internal.LogonTimeout}

    //     for _, test := range tests {
    //         s.Timeout(s.session, test)
    //         s.State(logoutState{})
    //     }
    // }

    // func (s *LogoutStateTestSuite) TestDisconnected() {
    //     s.MockApp.On("OnLogout").Return(nil)
    //     s.session.Disconnected(s.session)

    //     s.MockApp.AssertExpectations(s.T())
    //     s.State(latentState{})
    // }

    // func (s *LogoutStateTestSuite) TestFixMsgInNotLogout() {
    //     s.MockApp.On("FromApp").Return(nil)
    //     s.fixMsgIn(s.session, s.NewOrderSingle())

    //     s.MockApp.AssertExpectations(s.T())
    //     s.State(logoutState{})
    //     s.NextTargetMsgSeqNum(2)
    // }

    // func (s *LogoutStateTestSuite) TestFixMsgInNotLogoutReject() {
    //     s.MockApp.On("FromApp").Return(ConditionallyRequiredFieldMissing(Tag(11)))
    //     s.MockApp.On("ToApp").Return(nil)
    //     s.fixMsgIn(s.session, s.NewOrderSingle())

    //     s.MockApp.AssertExpectations(s.T())
    //     s.State(logoutState{})
    //     s.NextTargetMsgSeqNum(2)
    //     s.NextSenderMsgSeqNum(2)

    //     s.NoMessageSent()
    // }

    // func (s *LogoutStateTestSuite) TestFixMsgInLogout() {
    //     s.MockApp.On("FromAdmin").Return(nil)
    //     s.MockApp.On("OnLogout").Return(nil)
    //     s.fixMsgIn(s.session, s.Logout())

    //     s.MockApp.AssertExpectations(s.T())
    //     s.State(latentState{})
    //     s.NextTargetMsgSeqNum(2)
    //     s.NextSenderMsgSeqNum(1)
    //     s.NoMessageSent()
    // }

    // func (s *LogoutStateTestSuite) TestFixMsgInLogoutResetOnLogout() {
    //     s.session.ResetOnLogout = true

    //     s.MockApp.On("ToApp").Return(nil)
    //     s.Nil(s.queueForSend(s.NewOrderSingle()))
    //     s.MockApp.AssertExpectations(s.T())

    //     s.MockApp.On("FromAdmin").Return(nil)
    //     s.MockApp.On("OnLogout").Return(nil)
    //     s.fixMsgIn(s.session, s.Logout())

    //     s.MockApp.AssertExpectations(s.T())
    //     s.State(latentState{})
    //     s.NextTargetMsgSeqNum(1)
    //     s.NextSenderMsgSeqNum(1)

    //     s.NoMessageSent()
    //     s.NoMessageQueued()
    // }

    // func (s *LogoutStateTestSuite) TestStop() {
    //     s.session.Stop(s.session)
    //     s.State(logoutState{})
    //     s.NotStopped()
    // }
}
