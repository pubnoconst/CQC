open Spectre.Console
open StringMath

let greet () =
    printfn "Welcome to Quote calculator. Simply list the parts cost in each line."
    printfn "Pressing Enter twice will or entering a non-numerical line prompt me to compute the quote."

let getPrices () =
    let mutable prices = ResizeArray()
    let mutable loop = true
    let mutable i = 1

    while loop do
        try
            printf "Part %d cost: " i
            let cost = decimal <| System.Console.ReadLine().Eval()
            prices.Add cost
            i <- i + 1
        with _ ->
            printfn "END"
            loop <- false

    prices


let quote price =
    if price < 200M then price + 140M
    else if price <= 350M then price + 180M
    else System.Math.Max(price + 250M, price * 1.4M + 70M + 15M)


let printResult prices =
    let quotes = prices |> Seq.map (fun p -> quote p)
    let table = new Table()
    table.Border <- TableBorder.MinimalDoubleHead
    table.AddColumn("#").AddColumn("Part cost").AddColumn("Quote") |> ignore

    let mutable i = 1

    for cost, quote in quotes |> Seq.zip prices do
        table.AddRow($"{i}", sprintf "%.2f" cost, sprintf "%.2f" quote) |> ignore
        i <- i + 1

    let sum = Seq.sum quotes

    table.AddRow((new Markup "[yellow]Total[/]"), new Text(""), new Markup(sprintf "[bold yellow]%.2f[/]" sum))
    |> ignore

    AnsiConsole.Write(table)
    printfn "Mininum deposit: %.2f" <| sum / 2M


[<EntryPoint>]
let main _ =
    while true do
        AnsiConsole.Write(new Rule("[yellow]Quote Calculator[/]"))
        greet ()
        printResult <| getPrices ()
        System.Console.Beep()

    0

