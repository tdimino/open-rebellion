
void __thiscall FUN_00419580(void *this,uint *param_1)

{
  bool bVar1;
  void *pvVar2;
  
  if ((*param_1 >> 0x18 < 0x14) || (0x1b < *param_1 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    pvVar2 = FUN_004f5940((void *)((int)this + 0x58),param_1);
    if (pvVar2 != (void *)0x0) {
      *(uint *)((int)pvVar2 + 0x24) = *(uint *)((int)pvVar2 + 0x24) | 0x8000000;
      FUN_004306e0((void *)((int)this + 0x58),param_1);
    }
  }
  return;
}

