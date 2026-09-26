
undefined4 FUN_0055ea00(void)

{
  bool bVar1;
  int iVar2;
  int iVar3;
  
  iVar2 = FUN_0053e390(0x200,&DAT_006bb788);
  iVar3 = FUN_0053e390(0x201,&DAT_006bb780);
  if ((iVar3 == 0) || (iVar2 == 0)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar2 = FUN_0053e390(0x202,&DAT_006bb78c);
  if ((iVar2 == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar2 = FUN_0053e390(0x203,&DAT_006bb778);
  if ((iVar2 == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar2 = FUN_0053e390(0x204,&DAT_006bb784);
  if ((iVar2 == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar2 = FUN_0053e390(0x205,&DAT_006bb77c);
  if ((iVar2 != 0) && (bVar1)) {
    return 1;
  }
  return 0;
}

