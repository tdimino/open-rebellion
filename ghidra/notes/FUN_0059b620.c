
void __thiscall FUN_0059b620(void *this,int param_1)

{
  void *this_00;
  int iVar1;
  undefined4 *puVar2;
  uint *puVar3;
  uint uVar4;
  uint local_10 [4];
  
  FUN_00595bc0(local_10);
  uVar4 = 0;
  puVar3 = local_10;
  puVar2 = (undefined4 *)((int)this + 0xa8);
  do {
    this_00 = (void *)*puVar2;
    if ((*puVar3 == 1) && (this_00 != (void *)0x0)) {
      iVar1 = FUN_00602510((int)this_00);
      if (iVar1 == param_1) {
        FUN_00595b10(uVar4 + 0xb);
      }
      else {
        FUN_006030f0(this_00,5);
      }
    }
    uVar4 = uVar4 + 1;
    puVar2 = puVar2 + 1;
    puVar3 = puVar3 + 1;
  } while (uVar4 < 4);
  return;
}

