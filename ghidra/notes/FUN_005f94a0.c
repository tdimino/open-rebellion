
void __fastcall FUN_005f94a0(int *param_1)

{
  undefined4 *puVar1;
  void *this;
  void *this_00;
  uint uVar2;
  void *local_c;
  undefined1 *puStack_8;
  void *local_4;
  
  local_4 = (void *)0xffffffff;
  puStack_8 = &LAB_00655c0b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar1 = FUN_005f9860(param_1);
  if (puVar1 != (undefined4 *)0x0) {
    do {
      uVar2 = (puVar1[8] - (int)*(char *)((int)puVar1 + 0x27)) + 1;
      this = (void *)FUN_005f5500(param_1 + 0x1e,uVar2);
      if (this == (void *)0x0) {
        this_00 = (void *)FUN_00618b70(0x28);
        if (this_00 == (void *)0x0) {
          this = (void *)0x0;
        }
        else {
          local_4 = this;
          this = FUN_005fb410(this_00,uVar2);
        }
        local_4 = (void *)0xffffffff;
        FUN_005f5440(param_1 + 0x1e,this);
        if (this != (void *)0x0) goto LAB_005f9520;
      }
      else {
LAB_005f9520:
        FUN_005fb4a0(this,puVar1);
      }
      puVar1 = FUN_005f9860(param_1);
    } while (puVar1 != (undefined4 *)0x0);
  }
  ExceptionList = local_c;
  return;
}

