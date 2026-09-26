
undefined4 FUN_0055e840(void)

{
  bool bVar1;
  int iVar2;
  int iVar3;
  
  iVar2 = FUN_0053e390(0x600,&DAT_006bb768);
  iVar3 = FUN_0053e390(0x601,&DAT_006bb774);
  if ((iVar3 == 0) || (iVar2 == 0)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar2 = FUN_0053e390(0x602,&DAT_006bb75c);
  if ((iVar2 == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar2 = FUN_0053e390(0x603,&DAT_006bb764);
  if ((iVar2 == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar2 = FUN_0053e390(0x604,&DAT_006bb760);
  if ((iVar2 == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar2 = FUN_0053e390(0x605,&DAT_006bb770);
  if ((iVar2 == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar2 = FUN_0053e390(0x606,&DAT_006bb76c);
  if ((iVar2 != 0) && (bVar1)) {
    return 1;
  }
  return 0;
}

