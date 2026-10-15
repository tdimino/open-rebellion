
undefined4 FUN_0053ef00(undefined4 param_1,int param_2)

{
  bool bVar1;
  undefined4 uVar2;
  
  uVar2 = 0;
  if ((param_2 < 1) || (2 < param_2)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  if (bVar1) {
    if (param_2 == 1) {
      uVar2 = FUN_0053efd0(param_1,1);
      return uVar2;
    }
    uVar2 = FUN_0053efd0(param_1,2);
  }
  return uVar2;
}

