
void __cdecl FUN_0053a610(uint param_1,uint param_2,uint *param_3)

{
  if ((param_2 & param_1) != (param_1 & *param_3)) {
    *param_3 = ~param_1 & *param_3 | param_2 & param_1;
  }
  return;
}

