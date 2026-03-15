
void __thiscall FUN_00433320(void *this,uint *param_1)

{
  bool bVar1;
  void *pvVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062d18b;
  local_c = ExceptionList;
  if ((*param_1 >> 0x18 < 0x14) || (0x1b < *param_1 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  ExceptionList = &local_c;
  FUN_00619730();
  if (bVar1) {
    pvVar2 = FUN_004f5940((void *)((int)this + 0x48),param_1);
    if (pvVar2 == (void *)0x0) {
      pvVar2 = (void *)FUN_00618b70(0x1c);
      local_4 = 0;
      if (pvVar2 == (void *)0x0) {
        pvVar2 = (void *)0x0;
      }
      else {
        pvVar2 = FUN_004ecc70(pvVar2,param_1);
      }
      local_4 = 0xffffffff;
      FUN_004f57b0((void *)((int)this + 0x48),pvVar2);
      FUN_00433600((int)this);
    }
    FUN_00430670(*(void **)((int)this + 0x40),param_1,(undefined4 *)((int)this + 0x18));
  }
  ExceptionList = local_c;
  return;
}

