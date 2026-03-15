
void __thiscall FUN_004332b0(void *this,uint *param_1)

{
  bool bVar1;
  undefined4 *puVar2;
  
  if ((*param_1 >> 0x18 < 0x14) || (0x1b < *param_1 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    puVar2 = (undefined4 *)FUN_004f5910((void *)((int)this + 0x48),param_1);
    if (puVar2 != (undefined4 *)0x0) {
      (**(code **)*puVar2)(1);
      FUN_00433600((int)this);
    }
  }
  return;
}

