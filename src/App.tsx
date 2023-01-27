import LoginForm from './LoginForm'
import ShiftIdentity from './ShiftIdentity'
import {EmployeeProvider} from './employeeProvider'

function App() {
  return (
      <section>
        <ShiftIdentity/>
        <EmployeeProvider>
          <LoginForm/>
        </EmployeeProvider>
      </section>
  );
}

export default App;
