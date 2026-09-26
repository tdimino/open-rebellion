
void * __fastcall FUN_005fb180(undefined4 *param_1)

{
  ushort uVar1;
  char cVar2;
  void *pvVar3;
  uint uVar4;
  uint uVar5;
  undefined4 *puVar6;
  undefined4 *puVar7;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00655dcb;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  cVar2 = FUN_005fb0c0((int)param_1);
  if ('\0' < cVar2) {
    uVar1 = *(ushort *)((int)param_1 + 9);
    uVar5 = uVar1 + 9;
    if (uVar5 < 0x200) {
      pvVar3 = (void *)FUN_00618b70(0x220);
      local_4 = 0;
      if (pvVar3 == (void *)0x0) {
        pvVar3 = (void *)0x0;
      }
      else {
        pvVar3 = FUN_005fb760(pvVar3,(undefined4 *)((int)param_1 + 5),uVar5);
      }
      *(char *)(param_1 + 1) = *(char *)(param_1 + 1) + -1;
      puVar6 = (undefined4 *)(uVar1 + 0xe + (int)param_1);
      puVar7 = (undefined4 *)((int)param_1 + 5);
      for (uVar4 = 0x1ff - uVar5 >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
        *puVar7 = *puVar6;
        puVar6 = puVar6 + 1;
        puVar7 = puVar7 + 1;
      }
      for (uVar5 = 0x1ff - uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
        *(undefined1 *)puVar7 = *(undefined1 *)puVar6;
        puVar6 = (undefined4 *)((int)puVar6 + 1);
        puVar7 = (undefined4 *)((int)puVar7 + 1);
      }
      ExceptionList = local_c;
      return pvVar3;
    }
    (**(code **)*param_1)();
  }
  ExceptionList = local_c;
  return (void *)0x0;
}

