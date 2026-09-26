
int * __fastcall FUN_004c05e0(int param_1)

{
  int *piVar1;
  int iVar2;
  void *pvVar3;
  bool bVar4;
  undefined4 local_20;
  undefined4 local_1c;
  void *local_18;
  undefined4 local_14 [2];
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063a33a;
  pvStack_c = ExceptionList;
  pvVar3 = (void *)0x0;
  local_1c = 0;
  ExceptionList = &pvStack_c;
  FUN_004f4340(local_14);
  local_4 = 0;
  piVar1 = (int *)FUN_004f5cd0(0x201);
  if (piVar1 != (int *)0x0) {
    local_18 = (void *)FUN_00618b70(0x20);
    bVar4 = local_18 != (void *)0x0;
    if (bVar4) {
      local_20 = 0x20000005;
      local_1c = 1;
      local_4 = CONCAT31(local_4._1_3_,2);
      pvVar3 = FUN_004f5b10(local_18,&local_20,0);
    }
    local_4 = 1;
    if (bVar4) {
      FUN_00619730();
    }
    local_4 = local_4 & 0xffffff00;
    if (pvVar3 != (void *)0x0) {
      FUN_004f57b0(local_14,pvVar3);
      piVar1[8] = *(int *)(param_1 + 0x30);
      (**(code **)(*piVar1 + 0x24))(local_14);
      (**(code **)(*piVar1 + 0x2c))(param_1 + 0x50);
      iVar2 = thunk_FUN_004fcf00();
      *(int *)(param_1 + 0x54) = iVar2 + 300;
    }
  }
  local_4 = 0xffffffff;
  FUN_004f4380(local_14);
  ExceptionList = pvStack_c;
  return piVar1;
}

