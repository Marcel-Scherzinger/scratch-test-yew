use sreport::prelude::TestCaseStatus;
use yew::prelude::*;

#[derive(Properties, derive_more::PartialEq)]
pub struct TestCaseStatusIconProps {
    pub status: TestCaseStatus,
}

#[function_component(TestCaseStatusIconComponent)]
pub fn message_kind(TestCaseStatusIconProps { status }: &TestCaseStatusIconProps) -> Html {
    let icon = match status {
        TestCaseStatus::CompleteSucess => "check",
        TestCaseStatus::SuccessButWarnings => "check_alert",
        TestCaseStatus::Failure => "close",
    };
    html!(
        <span class="testcase-status-icon">
            <span class="material-symbols-outlined">
            {icon}
            </span>
        </span>
    )
}
