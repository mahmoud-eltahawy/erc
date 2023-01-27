import LoginForm from './LoginForm'
import ShiftIdentity from './ShiftIdentity'
import {EmployeeProvider} from './employeeProvider'
import Wall from './Wall';

function App() {
  return (
      <section>
        <ShiftIdentity/>
        <EmployeeProvider>
          <LoginForm/>
          <Wall>
            <button>اضافة عطل</button>
            <button>اضافة موظف</button>
            <button>اظهار الاعطال</button>
          </Wall>
        </EmployeeProvider>
      </section>
  );
}

export default App;
