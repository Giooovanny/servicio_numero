#![feature(proc_macro_hygiene, decl_macro)]
#![feature(str_split_as_str)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use serde::Deserialize;
use rocket_contrib::json::{Json, JsonValue};

#[derive(Deserialize)]
struct Numero {
    valor: String,
}

#[post("/numeroCardinal", format = "application/json", data = "<numero>")]
fn numero_a_palabra(numero: Json<Numero> ) -> JsonValue  {
  let mut parte_entera: String = "".to_string();
  let mut parte_decimal_letras: String = "".to_string();
  let mut ultimo_decimal: String = "".to_string();
  if numero.valor.contains(',') {
    let split = numero.valor.split(",");
    let vec = split.collect::<Vec<&str>>();
    parte_entera = convertir_numero(vec[0].to_string());
    parte_decimal_letras = convertir_numero(vec[1].to_string());
    ultimo_decimal = parte_decimal(vec[1].to_string()); 
  } else{
    parte_entera = convertir_numero(numero.valor.to_string());
  }

  json!({
    "status": 200,
    "message" : format!("{}enteros {}{}",parte_entera,parte_decimal_letras,ultimo_decimal),
  })
}

fn main() {
  rocket::ignite()
    .mount("/servicio", routes![numero_a_palabra])
    .launch();
}

fn convertir_numero(numero: String) -> String{
  let mut numero_letras: String = "".to_owned();

    let primeros_numeros: Vec<String>  = vec!["uno".to_string(), "dos".to_string(), "tres".to_string(), 
    "cuatro".to_string(), "cinco".to_string(), "seis".to_string(), "siete".to_string(), "ocho".to_string(), 
    "nueve".to_string(),"diez".to_string(), "once".to_string(), "doce".to_string(), "trece".to_string(), 
    "catorce".to_string(), "quince".to_string(), "dieciseis".to_string(), "diecisiete".to_string(), 
    "dieciocho".to_string(), "diecinueve".to_string(), "veinte".to_string()];

    let decenas: Vec<String>  = vec!["diez".to_string(), "veinte".to_string(), "treinta".to_string(),
    "cuarenta".to_string(), "cincuenta".to_string(), "sesenta".to_string(), "setenta".to_string(), 
    "ochenta".to_string(), "noventa".to_string()];

    let centenas: Vec<String>  = vec!["cien".to_string(), "doscientos".to_string(),
    "trecientos".to_string(), "cuatrocientos".to_string(), "quinientos".to_string(), "seiscientos".to_string(),
    "setecientos".to_string(), "ochocientos".to_string(), "novecientos".to_string(), "mil".to_string() ];
    
    let millones: Vec<String>  = vec!["millones".to_string(), "billones".to_string(), "trillones".to_string(),
     "cuatrillones".to_string(), "quintillones".to_string(), "sextillones".to_string(), "septillones".to_string(), 
     "octillones".to_string(), "nonillones".to_string(), "decillones".to_string(), "undecillones".to_string(), 
     "duodecillones".to_string(), "tredecillones".to_string(), "cuatordecillones".to_string(), 
     "quindecillones".to_string(), "sexdecillones".to_string(), "septendecillones".to_string(),
    "octodecillones".to_string(), "novendecillones".to_string(), "vigintillones".to_string()];
    
    let vector_numeros: Vec<char> = vec!['1','2','3','4','5','6','7','8','9'];
    
    let char_vec: Vec<char> = numero.chars().collect();
    let mut tamano: usize = char_vec.len();
    let tamano_inicial: usize = char_vec.len();
    let mut numero_valido: char =' ';
    
    for c in char_vec {
        let mut indice: usize = 0;
        if tamano%3 == 0{
            
            for a in &vector_numeros {
                if c == *a && c != '0' {
                    numero_letras.push_str(&centenas[indice]);
                    numero_letras.push_str(" ");
                }
                indice = indice + 1;
            }

        } else if tamano%3 == 2 {

            if c == '1' {
                numero_valido = c;
            }else {
                for a in &vector_numeros {
                    if c == *a && c != '0'  {
                        numero_letras.push_str(&decenas[indice]);
                        numero_letras.push_str(" ");
                    }
                    indice = indice + 1;
                }
            }

        } else if tamano%3 == 1 {

            if numero_valido == '1' {
                for a in &vector_numeros {
                    if c == *a {
                        
                        numero_letras.push_str(&primeros_numeros[indice+10]);
                        numero_letras.push_str(" ");
                      
                        if (tamano-1)%3 == 0 && (tamano-1)%2 != 0{//porque van de par y impar y multiplos de 3
                            numero_letras.push_str("mil ");
                        }else if (tamano-1)%6 == 0 && tamano>5{//para que no ponga en el ultimo un valor

                            numero_letras.push_str(&millones[(tamano-1)/6-1]);
                            numero_letras.push_str(" ");
                        }
                    }
                    indice = indice + 1;
                }
                if c == '0'{
                  numero_letras.push_str(&primeros_numeros[9]);
                  numero_letras.push_str(" ");
                  if (tamano-1)%3 == 0 && (tamano-1)%2 != 0{//porque van de par y impar y multiplos de 3
                      numero_letras.push_str("mil ");
                  }else if (tamano-1)%6 == 0 && tamano>5{//para que no ponga en el ultimo un valor

                      numero_letras.push_str(&millones[(tamano-1)/6-1]);
                      numero_letras.push_str(" ");
                  }
                }
                numero_valido = ' ';
            }else{
                for a in &vector_numeros {
                    if c == *a && c != '0'{
                        // para no imprimir 'y' antes
                        //if  tamano != 1 {
                            if tamano_inicial != tamano {
                                numero_letras.push_str("y ");
                            }

                            
                        //}
                        //para imprimir un embes de uno pero falta logica
                        // if c != '1' && tamano_inicial != tamano{
                          numero_letras.push_str(&primeros_numeros[indice]);
                          numero_letras.push_str(" ");
                        // }
                        if (tamano-1)%3 == 0 && (tamano-1)%2 != 0{
                            numero_letras.push_str("mil ");
                        }
                        else if (tamano-1)%6 == 0 && tamano>5{

                            numero_letras.push_str(&millones[(tamano-1)/6-1]);
                            numero_letras.push_str(" ");
                        }
                    } 
                    indice = indice + 1;
                }
                if c == '0'{
                    
                    if (tamano-1)%3 == 0 && (tamano-1)%2 != 0{
                        numero_letras.push_str("mil ");
                    }
                    else if (tamano-1)%6 == 0 && tamano>5{

                        numero_letras.push_str(&millones[(tamano-1)/6-1]);
                        numero_letras.push_str(" ");
                    }
                }
                numero_valido = ' ';
            }
        }
        tamano = tamano - 1;
      
    }
    return numero_letras
}

fn parte_decimal(numero: String) -> String{
  let mut numero_letras: String = "".to_owned();

  let decimales: Vec<String>  = vec!["decimas".to_string(), "centesimas".to_string(), "milesimas".to_string(),
    "millonesimas".to_string(), "milmillonésimas".to_string(), "billonésimas".to_string(), "milbillonésimas".to_string(), 
    "trillonésimas".to_string(), "miltrillonésimas".to_string(), "cuatrillonésimas".to_string()];

  let char_vec: Vec<char> = numero.chars().collect();
  let tamano: usize = char_vec.len();
  if tamano<4{
    numero_letras.push_str(&decimales[tamano-1]);
  }else if tamano %3 == 0{
    numero_letras.push_str(&decimales[tamano/3+1]);
  }else if tamano %3 == 1{
    numero_letras.push_str(" diez");
    numero_letras.push_str(&decimales[tamano/3+1]);
  }else if tamano %3 == 2{
    numero_letras.push_str(" cien");
    numero_letras.push_str(&decimales[tamano/3+1]);
  }
  return numero_letras
}