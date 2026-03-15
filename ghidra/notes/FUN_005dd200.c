
void __fastcall FUN_005dd200(int param_1)

{
  undefined4 uVar1;
  void *pvVar2;
  uint uVar3;
  undefined4 *puVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00654cab;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  uVar1 = FUN_006037f0(6);
  uVar3 = 0;
  puVar4 = (undefined4 *)(param_1 + 0xb0);
  do {
    pvVar2 = (void *)FUN_00618b70(0x28);
    local_4 = 0;
    if (pvVar2 == (void *)0x0) {
      pvVar2 = (void *)0x0;
    }
    else {
      pvVar2 = FUN_005fbd20(pvVar2,uVar1,uVar3 + 0x835,2);
    }
    *puVar4 = pvVar2;
    uVar3 = uVar3 + 1;
    puVar4 = puVar4 + 1;
    local_4 = 0xffffffff;
  } while (uVar3 < 5);
  ExceptionList = local_c;
  return;
}

