
int __cdecl FUN_00515cd0(uint *param_1,undefined4 param_2,void *param_3)

{
  void *this;
  int iVar1;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  int iVar2;
  bool bVar3;
  bool bVar4;
  void *local_4;
  
  this = (void *)thunk_FUN_00506e40();
  bVar3 = this != (void *)0x0;
  if (this != (void *)0x0) {
    iVar1 = FUN_00545d30(this,(int *)param_1,param_2,param_3);
    if ((iVar1 == 0) || (!bVar3)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
  }
  bVar4 = false;
  if (bVar3) {
    iVar1 = FUN_00505f60(param_1);
    bVar4 = iVar1 != 0;
  }
  local_4 = (void *)0x0;
  bVar3 = FUN_005187a0(1,(int *)&local_4);
  if ((CONCAT31(extraout_var,bVar3) == 0) || (!bVar4)) {
    bVar3 = false;
  }
  else {
    bVar3 = true;
  }
  if (local_4 != (void *)0x0) {
    iVar1 = FUN_0052fff0(local_4,param_3);
    if ((iVar1 == 0) || (!bVar3)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
  }
  bVar4 = FUN_005187a0(2,(int *)&local_4);
  if ((CONCAT31(extraout_var_00,bVar4) == 0) || (!bVar3)) {
    iVar1 = 0;
  }
  else {
    iVar1 = 1;
  }
  if (local_4 != (void *)0x0) {
    iVar2 = FUN_0052fff0(local_4,param_3);
    if ((iVar2 != 0) && (iVar1 != 0)) {
      return 1;
    }
    iVar1 = 0;
  }
  return iVar1;
}

