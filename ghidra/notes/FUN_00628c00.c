
int __cdecl FUN_00628c00(byte *param_1,LPWSTR param_2)

{
  byte *pbVar1;
  LPWSTR pWVar2;
  int *piVar3;
  
  pbVar1 = (byte *)*DAT_006be7a4;
  piVar3 = DAT_006be7a4;
  if (pbVar1 == (byte *)0x0) {
    return 0;
  }
  while ((pWVar2 = FUN_00628930(param_1,pbVar1,param_2), pWVar2 != (LPWSTR)0x0 ||
         ((*(char *)(*piVar3 + (int)param_2) != '=' && (*(char *)(*piVar3 + (int)param_2) != '\0')))
         )) {
    pbVar1 = (byte *)piVar3[1];
    piVar3 = piVar3 + 1;
    if (pbVar1 == (byte *)0x0) {
      return -((int)piVar3 - (int)DAT_006be7a4 >> 2);
    }
  }
  return (int)piVar3 - (int)DAT_006be7a4 >> 2;
}

