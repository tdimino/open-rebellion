
void * __thiscall FUN_004a2f80(void *this,void *param_1,int param_2,int param_3)

{
  short sVar1;
  undefined4 *puVar2;
  int iVar3;
  short sVar4;
  int iVar5;
  undefined4 local_24;
  undefined4 local_20;
  undefined4 local_1c [2];
  int local_14;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006379c7;
  local_c = ExceptionList;
  local_20 = 0;
  ExceptionList = &local_c;
  FUN_004f26d0(&local_24,(undefined4 *)((int)this + 0x144));
  local_4 = 1;
  sVar1 = (short)param_2;
  sVar4 = (short)param_3;
  iVar5 = CONCAT22(sVar4,sVar1);
  if ((((param_2 < 4) || (0x5f < param_2)) || (param_3 < 0x1d)) || (0x127 < param_3)) {
    if (((param_2 < 0x65) || (0xea < param_2)) || ((param_3 < 0x7f || (0x123 < param_3))))
    goto LAB_004a30eb;
    param_2 = CONCAT22(sVar4 + -0x7f,sVar1 + -0x65);
    puVar2 = (undefined4 *)FUN_00609d80(*(void **)((int)this + 0x164),&param_2,param_2);
    iVar3 = FUN_00609d10(*(void **)((int)this + 0x164),*puVar2);
    if (iVar3 == 0) {
      FUN_0060a790(local_1c,1);
      local_4 = CONCAT31(local_4._1_3_,2);
      iVar5 = 0;
      FUN_00609410(*(void **)((int)this + 0x160),local_1c);
      if (local_14 != 0) {
        iVar5 = FUN_0060a860(*(void **)((int)this + 0x188),*(int *)(local_14 + 0xc));
      }
      if (iVar5 != 0) {
        FUN_004f26d0(&local_24,(undefined4 *)(iVar5 + 0x6c));
      }
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_0060a810(local_1c);
      iVar5 = param_2;
      goto LAB_004a30eb;
    }
  }
  else {
    param_2 = CONCAT22(sVar4 + -0x1d,sVar1 + -4);
    puVar2 = (undefined4 *)FUN_00609d80(*(void **)((int)this + 0x160),&param_2,param_2);
    iVar3 = FUN_00609d10(*(void **)((int)this + 0x160),*puVar2);
    iVar5 = param_2;
    if (iVar3 == 0) goto LAB_004a30eb;
  }
  FUN_004f26d0(&local_24,(undefined4 *)(iVar3 + 0x6c));
  iVar5 = param_2;
LAB_004a30eb:
  param_2 = iVar5;
  FUN_004f26d0(param_1,&local_24);
  local_20 = 1;
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  ExceptionList = local_c;
  return param_1;
}

