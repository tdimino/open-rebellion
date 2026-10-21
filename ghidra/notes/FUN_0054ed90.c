
bool FUN_0054ed90(int param_1,int param_2)

{
  bool bVar1;
  
  bVar1 = true;
  if ((param_1 != 0) && (param_2 != 0)) {
    bVar1 = *(int *)(param_1 + 0x40) < *(int *)(param_2 + 0x40);
  }
  return bVar1;
}

