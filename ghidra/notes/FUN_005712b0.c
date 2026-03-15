
int __thiscall FUN_005712b0(void *this,int *param_1,int param_2,undefined4 *param_3,void *param_4)

{
  void *pvVar1;
  bool bVar2;
  undefined3 extraout_var;
  uint uVar3;
  int iVar4;
  int iVar5;
  int *this_00;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064c218;
  local_c = ExceptionList;
  iVar5 = 1;
  ExceptionList = &local_c;
  *param_3 = 0;
  bVar2 = FUN_0053e2f0(param_2);
  if (CONCAT31(extraout_var,bVar2) != 0) {
    local_4 = 0;
    uVar3 = (**(code **)(*param_1 + 4))();
    if ((uVar3 < 0x30) || (0x3b < uVar3)) {
      iVar5 = 0;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    this_00 = (int *)0x0;
    if (iVar5 != 0) {
      this_00 = param_1;
    }
    iVar4 = FUN_00505190((uint *)((int)this + 0xa8));
    if ((iVar4 == 0) || (iVar5 == 0)) {
      iVar5 = 0;
    }
    else {
      iVar5 = 1;
    }
    if ((this_00 != (int *)0x0) && (iVar4 != 0)) {
      iVar4 = FUN_0055ca20((int)(short)this_00[0x23],(int)*(short *)(iVar4 + 0x8c),(int *)&param_3);
      pvVar1 = param_4;
      if ((iVar4 == 0) || (iVar5 == 0)) {
        iVar5 = 0;
      }
      else {
        iVar5 = 1;
      }
      if ((undefined4 *)(int)(short)this_00[0x23] != param_3) {
        iVar4 = FUN_004ee1d0(this_00,(int)param_3 +
                                     ((int)(short)this_00[0x24] - (int)(short)this_00[0x23]),param_4
                            );
        if ((iVar4 == 0) || (iVar5 == 0)) {
          bVar2 = false;
        }
        else {
          bVar2 = true;
        }
        iVar5 = FUN_00521880(this,3,pvVar1);
        if ((iVar5 == 0) || (!bVar2)) {
          iVar5 = 0;
        }
        else {
          iVar5 = 1;
        }
      }
    }
  }
  ExceptionList = local_c;
  return iVar5;
}

