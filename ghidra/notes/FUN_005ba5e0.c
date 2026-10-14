
void FUN_005ba5e0(void)

{
  int iVar1;
  int iVar2;
  
  FUN_005aaf90();
  iVar1 = FUN_005a9330();
  if (iVar1 != 0) {
    iVar2 = FUN_005ba420(iVar1);
    if (iVar2 != 0) {
      *(undefined4 *)(iVar1 + 0x78) = 1;
      FUN_005b8d10(iVar1 + 0x1c);
      FUN_005c1200(*(uint *)(iVar1 + 0x18));
    }
  }
  return;
}

