
void __thiscall FUN_0042ff70(void *this,uint *param_1)

{
  bool bVar1;
  void *pvVar2;
  undefined4 uStack_18;
  
  if ((*param_1 >> 0x18 < 8) || (0xf < *param_1 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  uStack_18 = 0x42ffad;
  FUN_00619730();
  if (bVar1) {
    FUN_004f26d0(&uStack_18,param_1);
    pvVar2 = FUN_00419e40(this);
    if (pvVar2 != (void *)0x0) {
      uStack_18 = 0x42ffd0;
      FUN_00433600((int)pvVar2);
    }
  }
  return;
}

