
bool __thiscall FUN_00589970(void *this,uint param_1)

{
  bool bVar1;
  int iVar2;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  undefined3 extraout_var_02;
  undefined3 extraout_var_03;
  bool bVar3;
  
  iVar2 = FUN_00589a40(this,param_1);
  bVar3 = iVar2 != 0;
  if (*(int *)((int)this + 0x48) == 0) {
    bVar1 = FUN_00589e40(this,param_1);
    if ((CONCAT31(extraout_var,bVar1) == 0) || (!bVar3)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    bVar1 = FUN_00589f10(this,param_1);
    if ((CONCAT31(extraout_var_00,bVar1) == 0) || (!bVar3)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    bVar1 = FUN_0058a020(this,param_1);
    if ((CONCAT31(extraout_var_01,bVar1) == 0) || (!bVar3)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    bVar1 = FUN_0058a130(this,param_1);
    if ((CONCAT31(extraout_var_02,bVar1) == 0) || (!bVar3)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    bVar1 = FUN_0058a1c0(this,param_1);
    if ((CONCAT31(extraout_var_03,bVar1) == 0) || (!bVar3)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    iVar2 = FUN_0051ebb0();
    if ((iVar2 != 0) && (bVar3)) {
      return true;
    }
    bVar3 = false;
  }
  return bVar3;
}

