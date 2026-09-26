
uint __thiscall FUN_00567d90(void *this,uint param_1,uint param_2)

{
  undefined4 *puVar1;
  int iVar2;
  int iVar3;
  void *pvVar4;
  uint uVar5;
  undefined4 *local_20;
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064b2a7;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar1 = (undefined4 *)FUN_00618b70(0x3c);
  if (puVar1 == (undefined4 *)0x0) {
    FUN_00619730();
  }
  local_4 = 0;
  if (puVar1 == (undefined4 *)0x0) {
    local_20 = (undefined4 *)0x0;
  }
  else {
    local_20 = FUN_0058eb60(puVar1);
  }
  local_4 = 0xffffffff;
  iVar2 = 0;
  if ((local_20 != (undefined4 *)0x0) && (iVar2 = 0, local_20[7] != 0)) {
    iVar2 = FUN_0058ec20(local_20,param_1,param_2);
  }
  iVar3 = thunk_FUN_005f5060((int)(local_20 + 0xc));
  while ((iVar3 != 0 && (iVar2 != 0))) {
    param_2 = *(uint *)(iVar3 + 0x30);
    pvVar4 = FUN_005f2fc0(local_18,&param_2);
    local_4 = 1;
    FUN_005f3090((void *)(iVar3 + 0x34),(int)pvVar4);
    local_4 = 0xffffffff;
    FUN_005f2ff0(local_18);
    iVar3 = *(int *)(iVar3 + 0x10);
  }
  uVar5 = 0;
  if (iVar2 != 0) {
    uVar5 = FUN_005f5440(*(void **)((int)this + 8),local_20);
  }
  ExceptionList = local_c;
  return uVar5;
}

