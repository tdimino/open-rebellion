
undefined4 FUN_0055e340(void)

{
  bool bVar1;
  int iVar2;
  int iVar3;
  
  iVar2 = FUN_0053e390(0xe00,&DAT_006bb714);
  iVar3 = FUN_0053e390(0xe01,&DAT_006bb708);
  if ((iVar3 == 0) || (iVar2 == 0)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar2 = FUN_0053e390(0xe02,&DAT_006bb704);
  if ((iVar2 == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar2 = FUN_0053e390(0xe03,&DAT_006bb718);
  if ((iVar2 == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar2 = FUN_0053e390(0xe04,&DAT_006bb710);
  if ((iVar2 == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar2 = FUN_0053e390(0xe05,&DAT_006bb70c);
  if ((iVar2 != 0) && (bVar1)) {
    return 1;
  }
  return 0;
}

