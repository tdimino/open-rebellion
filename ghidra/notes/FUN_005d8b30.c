
int * __cdecl FUN_005d8b30(HRSRC param_1,int *param_2)

{
  undefined1 uVar1;
  int iVar2;
  int iVar3;
  HMODULE hModule;
  HGLOBAL hResData;
  int *piVar4;
  undefined1 *puVar5;
  undefined1 *puVar6;
  char cVar7;
  
  if (param_2 == (int *)0x0) {
    param_2 = (int *)FUN_00618b70(0x3c);
  }
  param_2[3] = 1;
  param_2[2] = 1;
  param_2[4] = 8;
  param_2[5] = 0;
  param_2[8] = 0;
  param_2[0xc] = 0;
  param_2[0xb] = 0;
  param_2[10] = 0;
  param_2[9] = 0;
  param_2[0xd] = 0x100;
  param_2[0xe] = DAT_006bcbd4 + 0x1bc;
  hModule = (HMODULE)FUN_005fefd0(6);
  hResData = LoadResource(hModule,param_1);
  piVar4 = LockResource(hResData);
  iVar2 = *piVar4;
  *param_2 = iVar2;
  param_2[6] = iVar2;
  iVar3 = piVar4[1];
  piVar4 = piVar4 + 2;
  param_2[1] = iVar3;
  puVar5 = (undefined1 *)FUN_00618b70(iVar2 * iVar3);
  param_2[7] = (int)puVar5;
  puVar6 = puVar5 + *param_2 * param_2[1];
  do {
    cVar7 = (char)*piVar4;
    uVar1 = *(undefined1 *)((int)piVar4 + 1);
    piVar4 = (int *)((int)piVar4 + 2);
    do {
      *puVar5 = uVar1;
      puVar5 = puVar5 + 1;
      cVar7 = cVar7 + -1;
    } while (cVar7 != '\0');
  } while (puVar5 < puVar6);
  FreeResource(hResData);
  return param_2;
}

