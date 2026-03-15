
undefined4 __thiscall FUN_00436690(void *this,int param_1,int param_2)

{
  undefined4 *puVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062d591;
  local_c = ExceptionList;
  if (param_1 == 1) {
    ExceptionList = &local_c;
    puVar1 = (undefined4 *)FUN_00618b70(0xc4);
    local_4 = 0;
    if (puVar1 == (undefined4 *)0x0) {
      puVar1 = (undefined4 *)0x0;
    }
    else {
      puVar1 = FUN_00486120(puVar1);
    }
    *(undefined4 **)((int)this + param_2 * 4 + 0x10) = puVar1;
    if (*(int *)((int)this + 0xc) == 0) {
      *(undefined4 **)((int)this + 0xc) = puVar1;
      ExceptionList = local_c;
      return 1;
    }
  }
  else {
    if (param_1 == 2) {
      ExceptionList = &local_c;
      puVar1 = (undefined4 *)FUN_00618b70(0x60);
      local_4 = 1;
      if (puVar1 != (undefined4 *)0x0) {
        puVar1 = FUN_00484df0(puVar1);
        *(undefined4 **)((int)this + param_2 * 4 + 0x10) = puVar1;
        ExceptionList = local_c;
        return 1;
      }
      *(undefined4 *)((int)this + param_2 * 4 + 0x10) = 0;
      ExceptionList = local_c;
      return 1;
    }
    if (param_1 == 3) {
      ExceptionList = &local_c;
      puVar1 = (undefined4 *)FUN_00618b70(0x42c);
      local_4 = 2;
      if (puVar1 != (undefined4 *)0x0) {
        puVar1 = FUN_004856f0(puVar1);
        *(undefined4 **)((int)this + param_2 * 4 + 0x10) = puVar1;
        ExceptionList = local_c;
        return 1;
      }
      *(undefined4 *)((int)this + param_2 * 4 + 0x10) = 0;
      ExceptionList = local_c;
      return 1;
    }
  }
  ExceptionList = local_c;
  return 0;
}

