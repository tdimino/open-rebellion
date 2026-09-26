
void __fastcall FUN_005df6b0(int param_1)

{
  undefined4 uVar1;
  int iVar2;
  void *pvVar3;
  uint uVar4;
  int iVar5;
  uint uVar6;
  undefined4 *puVar7;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00654d46;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  uVar1 = FUN_006037f0(6);
  iVar5 = 0;
  uVar4 = 4;
  iVar2 = FUN_00401060();
  if (iVar2 == 2) {
    iVar5 = 4;
    uVar4 = 3;
  }
  uVar6 = 0;
  if (uVar4 != 0) {
    puVar7 = (undefined4 *)(param_1 + 0xac);
    do {
      pvVar3 = (void *)FUN_00618b70(0x28);
      local_4 = 0;
      if (pvVar3 == (void *)0x0) {
        pvVar3 = (void *)0x0;
      }
      else {
        pvVar3 = FUN_005fbd20(pvVar3,uVar1,uVar6 + 0x867 + iVar5,2);
      }
      *puVar7 = pvVar3;
      uVar6 = uVar6 + 1;
      puVar7 = puVar7 + 1;
      local_4 = 0xffffffff;
    } while (uVar6 < uVar4);
  }
  pvVar3 = (void *)FUN_00618b70(0x28);
  local_4 = 1;
  if (pvVar3 == (void *)0x0) {
    pvVar3 = (void *)0x0;
  }
  else {
    pvVar3 = FUN_005fbd20(pvVar3,uVar1,0x86e,2);
  }
  *(void **)(param_1 + 0xbc) = pvVar3;
  ExceptionList = local_c;
  return;
}

