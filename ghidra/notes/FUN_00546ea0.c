
uint __thiscall FUN_00546ea0(void *this,uint *param_1,int param_2,int param_3,void *param_4)

{
  bool bVar1;
  bool bVar2;
  int *this_00;
  undefined3 extraout_var;
  int iVar3;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  uint uVar4;
  
  this_00 = (int *)FUN_00505970(param_1);
  uVar4 = (uint)(this_00 != (int *)0x0);
  if (this_00 != (int *)0x0) {
    bVar1 = FUN_00547f60(this_00,(uint)param_4);
    if ((CONCAT31(extraout_var,bVar1) == 0) || (uVar4 == 0)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    iVar3 = FUN_00548120(this_00,param_4);
    if ((iVar3 == 0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    if (param_3 == 2) {
      iVar3 = FUN_00548370(this_00,param_4);
      if ((iVar3 == 0) || (!bVar1)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
    }
    bVar2 = FUN_005484d0(this,this_00,param_2,param_3);
    if ((CONCAT31(extraout_var_00,bVar2) == 0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    bVar2 = FUN_00548840(this,this_00,param_2,param_3,param_4);
    if ((CONCAT31(extraout_var_01,bVar2) == 0) || (!bVar1)) {
      uVar4 = 0;
    }
    else {
      uVar4 = 1;
    }
    if (this_00 != (int *)0x0) {
      uVar4 = FUN_00522980(this_00,param_4);
    }
  }
  return uVar4;
}

