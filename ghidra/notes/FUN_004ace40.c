
void __thiscall FUN_004ace40(void *this,uint param_1)

{
  int iVar1;
  undefined4 *puVar2;
  undefined4 *this_00;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063870b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  iVar1 = FUN_005f5500((void *)((int)this + 0x1c),param_1);
  if (iVar1 == 0) {
    puVar2 = (undefined4 *)FUN_00618b70(0x1c);
    this_00 = (undefined4 *)0x0;
    local_4 = 0;
    if (puVar2 != (undefined4 *)0x0) {
      this_00 = FUN_005f5590(puVar2);
    }
    local_4 = 0xffffffff;
    FUN_005f5600(this_00,param_1);
    FUN_005f5440((void *)((int)this + 0x1c),this_00);
  }
  ExceptionList = local_c;
  return;
}

