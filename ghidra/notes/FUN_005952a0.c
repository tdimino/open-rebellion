
undefined4 __thiscall FUN_005952a0(void *this,uint param_1,uint param_2,char *param_3)

{
  void *pvVar1;
  char *pcVar2;
  int local_130;
  int aiStack_12c [22];
  int local_d4;
  int aiStack_d0 [2];
  undefined1 local_c8 [8];
  undefined4 auStack_c0 [20];
  int local_70;
  int aiStack_6c [2];
  undefined1 local_64 [8];
  undefined4 auStack_5c [20];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00650c9e;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_00616620(&local_130,1);
  local_4 = 0;
  FUN_00615db0(&local_70,1);
  local_4._0_1_ = 1;
  FUN_00615db0(&local_d4,1);
  local_4 = CONCAT31(local_4._1_3_,2);
  if (param_3 != (char *)0x0) {
    pcVar2 = &DAT_006a7c4c;
    pvVar1 = FUN_00616110(local_64,param_3);
    FUN_00616110(pvVar1,pcVar2);
  }
  pcVar2 = FUN_005f49e0(param_1 & 0xffff);
  FUN_00616110(local_c8,pcVar2);
  pcVar2 = FUN_005f49e0(param_2 & 0xffff);
  pvVar1 = FUN_00616110(local_c8,pcVar2);
  FUN_00615f00(pvVar1,0);
  pcVar2 = (char *)FUN_00615680(*(int *)((int)aiStack_d0 + *(int *)(local_d4 + 4)));
  FUN_005f4a60(&local_130,pcVar2);
  if (*(int *)(*(int *)((int)aiStack_12c + *(int *)(local_130 + 4)) + 0x4c) == -1) {
    FUN_00616110(local_64,pcVar2);
    pcVar2 = (char *)FUN_00615680(*(int *)((int)aiStack_6c + *(int *)(local_70 + 4)));
    FUN_005f4a60(&local_130,pcVar2);
    FUN_00615660(*(void **)((int)aiStack_6c + *(int *)(local_70 + 4)),0);
  }
  FUN_00615660(*(void **)((int)aiStack_d0 + *(int *)(local_d4 + 4)),0);
  if (*(int *)(*(int *)((int)aiStack_12c + *(int *)(local_130 + 4)) + 0x4c) != -1) {
    (*(code *)**(undefined4 **)this)(&local_130);
  }
  FUN_005f4bc0(&local_130);
  local_4._0_1_ = 1;
  FUN_00615ea0((int)auStack_c0);
  FUN_00615440(auStack_c0);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00615ea0((int)auStack_5c);
  FUN_00615440(auStack_5c);
  local_4 = 0xffffffff;
  FUN_00616810((int)(aiStack_12c + 2));
  FUN_00615440(aiStack_12c + 2);
  ExceptionList = pvStack_c;
  return 1;
}

