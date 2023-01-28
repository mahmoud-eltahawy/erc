import LoginForm from './LoginForm'
import ShiftIdentity from './ShiftIdentity'
import {EmployeeProvider} from './employeeProvider'
import Wall from './Wall';
import { SearchBar } from './SearchBar';
import { Name } from './main';

const list : Name[] = [
    {id : '1',name : 'mahmoud'},
    {id : '2',name : 'gamal'},
    {id : '3',name : 'mohammed'},
    {id : '4',name : 'abbas'},
    {id : '5',name : 'toliba'},
    {id : '6',name : 'tahawy'},
    {id : '7',name : 'salem'},
    {id : '8',name : 'ali'},
    {id : '9',name : 'otman'},
    {id : '10',name : 'omar'}
]

function App() {
  return (
      <section>
      <SearchBar
          defaultPlaceholder='search here'
          isMulti={true}
          mtMessage='empty message'
          nyMessage='not yet message'
          resultPlaceholder='result placeholder'
          optionsList={list}/>
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
