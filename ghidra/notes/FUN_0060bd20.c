
void __thiscall FUN_0060bd20(void *this,short param_1,short param_2,undefined4 param_3)

{
  undefined4 *puVar1;
  void *pvVar2;
  undefined2 extraout_var;
  undefined2 extraout_var_00;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00656a76;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  if (*(short *)((int)this + 0x10) != param_1) {
    puVar1 = *(undefined4 **)((int)this + 0x20);
    ExceptionList = &local_c;
    *(short *)((int)this + 0x10) = param_1;
    if (puVar1 != (undefined4 *)0x0) {
      FUN_005fbfa0(puVar1);
      FUN_00618b60((undefined *)puVar1);
    }
    if (*(short *)((int)this + 0x10) == 0) {
      *(undefined4 *)((int)this + 0x20) = 0;
    }
    else {
      pvVar2 = (void *)FUN_00618b70(0x28);
      local_4 = 0;
      if (pvVar2 == (void *)0x0) {
        local_4 = 0xffffffff;
        *(undefined4 *)((int)this + 0x20) = 0;
      }
      else {
        pvVar2 = FUN_005fbd20(pvVar2,param_3,
                              CONCAT22(extraout_var,*(undefined2 *)((int)this + 0x10)),2);
        local_4 = 0xffffffff;
        *(void **)((int)this + 0x20) = pvVar2;
      }
    }
  }
  if (*(short *)((int)this + 0x12) != param_2) {
    puVar1 = *(undefined4 **)((int)this + 0x24);
    *(short *)((int)this + 0x12) = param_2;
    if (puVar1 != (undefined4 *)0x0) {
      FUN_005fbfa0(puVar1);
      FUN_00618b60((undefined *)puVar1);
    }
    if (*(short *)((int)this + 0x12) != 0) {
      pvVar2 = (void *)FUN_00618b70(0x28);
      local_4 = 1;
      if (pvVar2 != (void *)0x0) {
        pvVar2 = FUN_005fbd20(pvVar2,param_3,
                              CONCAT22(extraout_var_00,*(undefined2 *)((int)this + 0x12)),2);
        *(void **)((int)this + 0x24) = pvVar2;
        ExceptionList = local_c;
        return;
      }
      *(undefined4 *)((int)this + 0x24) = 0;
      ExceptionList = local_c;
      return;
    }
    *(undefined4 *)((int)this + 0x24) = 0;
  }
  ExceptionList = local_c;
  return;
}

