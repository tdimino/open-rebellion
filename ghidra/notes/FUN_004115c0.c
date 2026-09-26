
void __fastcall FUN_004115c0(int param_1)

{
  bool bVar1;
  DWORD DVar2;
  int iVar3;
  undefined3 extraout_var;
  undefined4 *puVar4;
  int *piVar5;
  undefined4 local_28;
  int local_24;
  uint local_14;
  undefined4 *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062a687;
  local_c = ExceptionList;
  if ((*(int *)(param_1 + 0x164) != 0) && (*(int *)(param_1 + 0x144) != 0)) {
    ExceptionList = &local_c;
    DVar2 = timeGetTime();
    if (DAT_006b1248 <= DVar2 - 1000) {
      iVar3 = FUN_00597090();
      if (*(int *)(param_1 + 0x11c) != 0) {
        if (iVar3 == 2) {
          bVar1 = FUN_00603120(*(void **)(param_1 + 0x144),2);
          if (CONCAT31(extraout_var,bVar1) == 0) {
            DAT_006b1248 = 0xffffffff;
            ExceptionList = local_c;
            return;
          }
          puVar4 = (undefined4 *)FUN_00618b70(0x3c);
          if (puVar4 == (undefined4 *)0x0) {
            FUN_00619730();
          }
          local_4 = 0;
          if (puVar4 == (undefined4 *)0x0) {
            piVar5 = (int *)0x0;
          }
          else {
            piVar5 = FUN_00420bd0(puVar4);
          }
          local_4 = 0xffffffff;
          if (piVar5 != (int *)0x0) {
            FUN_0051c300(&local_28);
            local_4 = 1;
            FUN_0051c410(&local_28,piVar5);
            if (local_24 != 0) {
              FUN_005970d0(local_10,local_14,1);
              FUN_00597170();
            }
            (**(code **)*piVar5)(1);
            local_4 = 0xffffffff;
            FUN_0051c350(&local_28);
          }
        }
        DAT_006b1248 = timeGetTime();
      }
    }
  }
  ExceptionList = local_c;
  return;
}

