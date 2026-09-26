
undefined * __thiscall FUN_005fa050(void *this,undefined4 param_1,int *param_2)

{
  int iVar1;
  undefined4 local_60;
  undefined4 local_5c;
  undefined4 local_58;
  undefined4 local_54;
  undefined4 local_50 [20];
  
  *param_2 = 3;
  if ((*(int *)((int)this + 0x108) == 0) || (iVar1 = FUN_005fa7a0(), iVar1 != 0)) goto LAB_005fa139;
  FUN_005f5540(&DAT_006be330);
  iVar1 = FUN_005fa2b0(this,local_50,local_60,local_5c,local_58,local_54,0,0);
  *param_2 = iVar1;
  if (iVar1 != 0) goto LAB_005fa139;
  DAT_006be3a4 = 1;
  iVar1 = (**(code **)(**(int **)((int)this + 0x108) + 0x34))
                    (*(int **)((int)this + 0x108),local_50,param_1,&LAB_005f8950,0,1);
  if (iVar1 == 0) {
    *param_2 = 0;
    goto LAB_005fa139;
  }
  if (iVar1 < -0x7788fffa) {
    if (iVar1 != -0x7788fffb) {
      if (iVar1 != -0x7ff8ffa9) goto LAB_005fa139;
      goto LAB_005fa118;
    }
  }
  else {
    if (iVar1 != -0x7788ff7e) {
      if ((iVar1 == -0x7788fee8) || (iVar1 == -0x7788fea2)) {
        *param_2 = 8;
      }
      goto LAB_005fa139;
    }
LAB_005fa118:
    FUN_005f8600(this);
    FUN_005ae1e0();
  }
  *param_2 = 6;
LAB_005fa139:
  if (*param_2 == 0) {
    DAT_006be3a4 = 0;
    return &DAT_006be330;
  }
  DAT_006be3a4 = 1;
  FUN_005f5540(&DAT_006be330);
  return &DAT_006be330;
}

