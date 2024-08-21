type Table = {
  name: string,
  age: number,
  fn: function(string): string,
}

local person: Table = {
  name = "Stella",
  age = 25,
  fn = function(name: string): string
    return "Hello, " .. name
  end
}

print(person.name)
print(person.age)
print(person.fn("Alice"))



-- list {1, 2, 3 , 'a', 'b', 'c'}

type List = { number | string }
local list: List = {1, 2, 3, 'a', 'b', 'c'}
print(list[1])
print(list[2])
print(list[3])
print(list[4])


-- recusive type

type LinkedList = {
  value: number,
  next: Option<LinkedList>
}

local list: LinkedList = {
  value = 10,
  next = nil
}

list.next = {
  value = 20,
  next = nil
}


-- type(list) -> table { value = number, next =  nil }


list.next.next = {
  value = 30,
  next = nil
}

print(list.value)
print(list.next.value)
print(list.next.next.value)



-- generics
type Pair<T, U> = {
  first: T,
  second: U
}

local pair: Pair<number, string> = {
  first = 10,
  second = "Hello"
}

pair.first = 20
pair.second = "World"

print(pair.first)
print(pair.second)

pair.first = pair.second
