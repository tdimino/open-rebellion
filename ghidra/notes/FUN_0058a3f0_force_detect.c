
int __thiscall FUN_0058a3f0(void *this,uint *param_1,uint *param_2,int param_3)

{
  bool bVar1;
  bool bVar2;
  void *pvVar3;
  int *piVar4;
  int iVar5;
  undefined3 extraout_var;
  int iVar6;
  int local_8 [2];
  
  pvVar3 = (void *)FUN_00505190(param_2);
  piVar4 = (int *)FUN_00505190(param_1);
  if ((pvVar3 == (void *)0x0) || (piVar4 == (int *)0x0)) {
    iVar6 = 0;
  }
  else {
    iVar6 = 1;
  }
  local_8[0] = 0;
  if ((pvVar3 != (void *)0x0) && (piVar4 != (int *)0x0)) {
    local_8[1] = 0;
    iVar5 = FUN_0055e4d0((uint)piVar4[9] >> 6 & 3,(int)(short)piVar4[0x23],
                         (int)*(short *)((int)pvVar3 + 0x8c),local_8,local_8 + 1);
    if ((iVar5 == 0) || (iVar6 == 0)) {
      iVar6 = 0;
    }
    else {
      iVar6 = 1;
    }
    FUN_005897b0();
  }
  if (((local_8[0] != 0) && (pvVar3 != (void *)0x0)) && (piVar4 != (int *)0x0)) {
    if ((*(byte *)(piVar4 + 0x1e) & 1) == 0) {
      *(undefined4 *)((int)this + 0x4c) = 1;
    }
    iVar5 = thunk_FUN_00506e80();
    if ((iVar5 == 0) || (iVar6 == 0)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    if (iVar5 != 0) {
      bVar2 = FUN_0055ff60(piVar4,pvVar3,param_3);
      if ((CONCAT31(extraout_var,bVar2) == 0) || (!bVar1)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
    }
    iVar6 = FUN_0058a530(this,param_1,param_2);
    if ((iVar6 != 0) && (bVar1)) {
      return 1;
    }
    iVar6 = 0;
  }
  return iVar6;
}

