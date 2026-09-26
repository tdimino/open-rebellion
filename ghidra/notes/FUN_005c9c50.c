
void __thiscall FUN_005c9c50(void *this,int param_1)

{
  undefined4 *puVar1;
  int iVar2;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00653f76;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  if (*(undefined4 **)((int)this + 0x40) != (undefined4 *)0x0) {
    ExceptionList = &pvStack_c;
    (**(code **)**(undefined4 **)((int)this + 0x40))(1);
  }
  *(undefined4 *)((int)this + 0x40) = 0;
  *(int *)((int)this + 0x3c) = param_1;
  if (param_1 == 1) {
    puVar1 = (undefined4 *)FUN_00618b70(0x3c);
    local_4 = 1;
    if (puVar1 == (undefined4 *)0x0) {
      puVar1 = (undefined4 *)0x0;
    }
    else {
      FUN_005f5c10(puVar1 + 6);
      puVar1[6] = &PTR_FUN_0066c678;
      *puVar1 = &PTR_FUN_0066ccf0;
      puVar1[1] = 0;
      puVar1[5] = 0;
      puVar1[4] = 0;
      puVar1[3] = DAT_0066cc94;
      puVar1[2] = this;
      puVar1[0xc] = 0;
      puVar1[0xd] = puVar1[3];
      puVar1[0xe] = 0;
      *puVar1 = &PTR_FUN_0066ccb0;
      puVar1[1] = 1;
    }
    local_4 = 0xffffffff;
    *(undefined4 **)((int)this + 0x40) = puVar1;
    iVar2 = (**(code **)(*(int *)this + 4))();
    if (iVar2 != 3) {
      ExceptionList = pvStack_c;
      return;
    }
    iVar2 = 1;
  }
  else {
    puVar1 = (undefined4 *)FUN_00618b70(0x3c);
    local_4 = 0;
    if (puVar1 == (undefined4 *)0x0) {
      puVar1 = (undefined4 *)0x0;
    }
    else {
      FUN_005f5c10(puVar1 + 6);
      puVar1[6] = &PTR_FUN_0066c678;
      *puVar1 = &PTR_FUN_0066ccf0;
      puVar1[1] = 0;
      puVar1[5] = 0;
      puVar1[4] = 0;
      puVar1[3] = DAT_0066cc94;
      puVar1[2] = this;
      puVar1[0xc] = 0;
      puVar1[0xd] = puVar1[3];
      puVar1[0xe] = 0;
      *puVar1 = &PTR_FUN_0066ccd0;
      puVar1[1] = 2;
    }
    local_4 = 0xffffffff;
    *(undefined4 **)((int)this + 0x40) = puVar1;
    iVar2 = (**(code **)(*(int *)this + 4))();
    if (iVar2 != 3) {
      ExceptionList = pvStack_c;
      return;
    }
    iVar2 = 2;
  }
  FUN_005cb1a0(this,iVar2);
  ExceptionList = pvStack_c;
  return;
}

