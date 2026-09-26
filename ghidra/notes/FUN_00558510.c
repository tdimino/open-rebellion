
undefined4 FUN_00558510(void)

{
  bool bVar1;
  int iVar2;
  int iVar3;
  
  iVar2 = FUN_0053e390(0x800,&DAT_006bb388);
  iVar3 = FUN_0053e390(0x801,&DAT_006bb378);
  if ((iVar3 == 0) || (iVar2 == 0)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar2 = FUN_0053e390(0x802,&DAT_006bb380);
  if ((iVar2 == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar2 = FUN_0053e390(0x803,&DAT_006bb384);
  if ((iVar2 == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar2 = FUN_0053e390(0x804,&DAT_006bb374);
  if ((iVar2 == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar2 = FUN_0053e390(0x805,&DAT_006bb37c);
  if ((iVar2 != 0) && (bVar1)) {
    return 1;
  }
  return 0;
}

