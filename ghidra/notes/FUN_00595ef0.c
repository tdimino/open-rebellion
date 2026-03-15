
void __cdecl
FUN_00595ef0(undefined4 param_1,undefined4 param_2,undefined4 param_3,undefined4 param_4,
            undefined4 param_5,undefined4 param_6,undefined4 param_7)

{
  int iVar1;
  void *this;
  
  iVar1 = FUN_00597530();
  if (*(int *)(iVar1 + 0x668) != 0) {
    FUN_00596f30();
    FUN_005af4e0(param_1,param_2,param_4,param_3,param_5,param_6,param_7);
    return;
  }
  this = (void *)FUN_00597530();
  FUN_0059a750(this,param_1,param_2,param_4,param_3,param_5,param_6,param_7);
  return;
}

