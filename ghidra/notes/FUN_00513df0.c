
bool __cdecl FUN_00513df0(int *param_1,int *param_2)

{
  bool bVar1;
  undefined3 extraout_var;
  int *this;
  int iVar2;
  int iVar3;
  
  bVar1 = FUN_0053f610(param_1);
  bVar1 = CONCAT31(extraout_var,bVar1) != 0;
  iVar3 = 0;
  do {
    this = param_2;
    if (iVar3 != 0) {
      this = (int *)FUN_00539fd0(param_2,iVar3);
    }
    if ((this == (int *)0x0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    if (this != (int *)0x0) {
      iVar2 = (**(code **)(*this + 0x18))(param_1);
      if ((iVar2 == 0) || (!bVar1)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      iVar2 = FUN_0053a350(this,param_1);
      if ((iVar2 == 0) || (!bVar1)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
    }
    iVar3 = iVar3 + 1;
  } while (iVar3 < 3);
  return bVar1;
}

