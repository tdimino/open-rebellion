
uint __fastcall FUN_0059f610(void *param_1)

{
  int iVar1;
  uint uVar2;
  
  uVar2 = 0;
  do {
    iVar1 = FUN_0059f5d0(param_1,uVar2);
    if (iVar1 == 0) {
      return uVar2;
    }
    uVar2 = uVar2 + 1;
  } while (uVar2 < 8);
  return 0xffffffff;
}

