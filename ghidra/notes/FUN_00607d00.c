
void __thiscall FUN_00607d00(void *param_1,int param_2)

{
  if (param_2 == 5) {
    *(undefined4 *)((int)param_1 + 0x110) = 1;
    FUN_005ffe40(param_1,5);
    return;
  }
  if (param_2 == 0) {
    *(undefined4 *)((int)param_1 + 0x110) = 0;
  }
  FUN_005ffe40(param_1,param_2);
  return;
}

