
uint __thiscall FUN_00599c40(int param_1,uint param_2)

{
  uint uVar1;
  uint *puVar2;
  
  uVar1 = 0;
  puVar2 = (uint *)(param_1 + 0x268);
  do {
    if (*puVar2 == (param_2 & 0xfff)) {
      return uVar1;
    }
    uVar1 = uVar1 + 1;
    puVar2 = puVar2 + 1;
  } while (uVar1 < 0x100);
  return 0;
}

