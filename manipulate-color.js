const incBt = document.getElementById('increment');
const decBt = document.getElementById('decrement');
let counter = 0;
const ulEl = document.getElementById('list-items')

function inCounter(){
    const counterEl = document.getElementById('counter')
    counter++
    counterEl.innerText = counter

    const li = document.createElement('li')
    li.setAttribute('data-counter', counter)

    if(counter % 2 ===0){
        li.style.background = 'red'
    } else {
        li.style.background = 'yellow'
    }
    li.innerHTML = '<b>Something </b>' + counter
    

    ulEl.appendChild(li)
}

function decCounter(){

    const li = ulEl.querySelector('[data-counter="'+counter+'"]')
    const number = parseInt(li.getAttribute('data-counter'), 10)
    if(number % 2 === 0){
    li.remove()}

    const counterEl = document.getElementById('counter')
    counter--
    counterEl.innerText = counter
}
incBt.addEventListener('click',inCounter)
decBt.addEventListener('click',decCounter)
