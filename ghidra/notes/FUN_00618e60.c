
void __cdecl FUN_00618e60(undefined *param_1)

{
  undefined *lpMem;
  byte *pbVar1;
  int local_4;
  
  lpMem = param_1;
  if (param_1 != (undefined *)0x0) {
    FUN_0061d9d0(9);
    pbVar1 = (byte *)FUN_0061ddf0(lpMem,&local_4,(uint *)&param_1);
    if (pbVar1 != (byte *)0x0) {
      FUN_0061de50(local_4,(int)param_1,pbVar1);
      FUN_0061da50(9);
      return;
    }
    FUN_0061da50(9);
    HeapFree(DAT_006bed14,0,lpMem);
  }
  return;
}

