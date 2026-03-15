
void __thiscall FUN_00419740(void *this,uint *param_1)

{
  bool bVar1;
  void *pvVar2;
  undefined4 uStack_18;
  
  if ((*param_1 >> 0x18 < 0x90) || (0x97 < *param_1 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  uStack_18 = 0x419781;
  FUN_00619730();
  if (bVar1) {
    FUN_004f26d0(&uStack_18,param_1);
    pvVar2 = FUN_00403d30((void *)((int)this + 0x2c));
    if (pvVar2 != (void *)0x0) {
      uStack_18 = 0x4197a5;
      FUN_00433dc0((int)pvVar2);
    }
  }
  return;
}

