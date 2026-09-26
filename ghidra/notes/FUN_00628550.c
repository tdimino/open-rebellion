
int __cdecl FUN_00628550(byte *param_1)

{
  byte bVar1;
  LPCWSTR pWVar2;
  LPWSTR pWVar3;
  uint uVar4;
  LPWSTR pWVar5;
  byte *pbVar6;
  int *piVar7;
  byte *pbVar8;
  
  if (((DAT_006be7a4 != (int *)0x0) ||
      (((DAT_006be7ac == 0 || (pWVar2 = FUN_00628970(), pWVar2 == (LPCWSTR)0x0)) &&
       (DAT_006be7a4 != (int *)0x0)))) && (param_1 != (byte *)0x0)) {
    uVar4 = 0xffffffff;
    pbVar6 = (byte *)*DAT_006be7a4;
    pbVar8 = param_1;
    do {
      if (uVar4 == 0) break;
      uVar4 = uVar4 - 1;
      bVar1 = *pbVar8;
      pbVar8 = pbVar8 + 1;
    } while (bVar1 != 0);
    pWVar5 = (LPWSTR)(~uVar4 - 1);
    piVar7 = DAT_006be7a4;
    if (pbVar6 != (byte *)0x0) {
      do {
        uVar4 = 0xffffffff;
        pbVar8 = pbVar6;
        do {
          if (uVar4 == 0) break;
          uVar4 = uVar4 - 1;
          bVar1 = *pbVar8;
          pbVar8 = pbVar8 + 1;
        } while (bVar1 != 0);
        if (((pWVar5 < (LPWSTR)(~uVar4 - 1)) && (*(byte *)((int)pWVar5 + (int)pbVar6) == 0x3d)) &&
           (pWVar3 = FUN_00628930(pbVar6,param_1,pWVar5), pWVar3 == (LPWSTR)0x0)) {
          return *piVar7 + 1 + (int)pWVar5;
        }
        pbVar6 = (byte *)piVar7[1];
        piVar7 = piVar7 + 1;
        if (pbVar6 == (byte *)0x0) {
          return 0;
        }
      } while( true );
    }
  }
  return 0;
}

