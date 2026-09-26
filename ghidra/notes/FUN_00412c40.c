
void __fastcall FUN_00412c40(void *param_1)

{
  uint uVar1;
  int *piVar2;
  HMODULE hModule;
  HRSRC hResInfo;
  HGLOBAL hResData;
  void *pvVar3;
  int iVar4;
  undefined2 extraout_var;
  undefined2 extraout_var_00;
  int iVar5;
  int iVar6;
  uint *puVar7;
  short *psStack_30;
  short *local_2c;
  int iStack_28;
  undefined4 uStack_20;
  int local_1c;
  void *pvStack_18;
  undefined2 local_14;
  ushort uStack_12;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0062a8ec;
  pvStack_c = ExceptionList;
  uVar1 = *(uint *)((int)param_1 + 0x14);
  local_1c = *(int *)((int)param_1 + 0x10);
  local_14 = (undefined2)uVar1;
  uStack_12 = (ushort)(uVar1 >> 0x10);
  local_2c = (short *)0x0;
  ExceptionList = &pvStack_c;
  hModule = (HMODULE)FUN_005fefd0((uint)uStack_12);
  hResInfo = FindResourceA(hModule,(LPCSTR)(uVar1 & 0xffff),(LPCSTR)0xa);
  hResData = uStack_20;
  if ((hResInfo != (HRSRC)0x0) &&
     (hResData = LoadResource(hModule,hResInfo), uStack_20 = hResData, hResData != (HGLOBAL)0x0)) {
    local_2c = LockResource(hResData);
  }
  if (local_2c != (short *)0x0) {
    iVar5 = 0;
    *(uint *)((int)param_1 + 0x4c) = (uint)(ushort)local_2c[3];
    if (0 < *(int *)((int)param_1 + 0x48)) {
      iStack_28 = 0;
      psStack_30 = local_2c + 4;
      do {
        pvStack_18 = (void *)FUN_00618b70(0x28);
        uStack_4 = 0;
        if (pvStack_18 == (void *)0x0) {
          pvVar3 = (void *)0x0;
        }
        else {
          pvVar3 = FUN_005fbd20(pvStack_18,hModule,CONCAT22(extraout_var,*psStack_30),2);
        }
        iVar6 = 1;
        *(void **)(*(int *)((int)param_1 + 0x54) + iVar5 * 4) = pvVar3;
        uStack_4 = 0xffffffff;
        if (1 < *(int *)((int)param_1 + 0x4c)) {
          do {
            pvStack_18 = (void *)FUN_00618b70(0x3c);
            uStack_4 = 1;
            if (pvStack_18 == (void *)0x0) {
              pvVar3 = (void *)0x0;
            }
            else {
              pvVar3 = FUN_0041c590(pvStack_18,hModule,
                                    CONCAT22(extraout_var_00,(short)iVar6 + *psStack_30));
            }
            uStack_4 = 0xffffffff;
            FUN_005f5600(pvVar3,iVar6);
            FUN_005f5440((void *)(iStack_28 + *(int *)((int)param_1 + 0x58)),pvVar3);
            iVar6 = iVar6 + 1;
          } while (iVar6 < *(int *)((int)param_1 + 0x4c));
        }
        pvVar3 = (void *)FUN_00618b70(0x28);
        uStack_4 = 2;
        pvStack_18 = pvVar3;
        if (pvVar3 == (void *)0x0) {
          pvVar3 = (void *)0x0;
        }
        else {
          puVar7 = *(uint **)(*(int *)((int)param_1 + 0x54) + iVar5 * 4);
          iVar6 = FUN_005fc0f0((int *)puVar7);
          iVar4 = FUN_005fc0e0(*(int **)(*(int *)((int)param_1 + 0x54) + iVar5 * 4));
          pvVar3 = FUN_005fbda0(pvVar3,iVar4,iVar6,puVar7);
        }
        *(void **)(*(int *)((int)param_1 + 0x60) + iVar5 * 4) = pvVar3;
        uStack_4 = 0xffffffff;
        FUN_005fcc30(*(void **)(*(int *)((int)param_1 + 0x54) + iVar5 * 4),
                     *(uint **)(*(int *)((int)param_1 + 0x60) + iVar5 * 4),0,0,0,0,(undefined4 *)0x0
                     ,0);
        piVar2 = *(int **)(*(int *)(local_1c + 0x20) + iVar5 * 4);
        if (piVar2 == (int *)0x0) {
          pvStack_18 = (void *)FUN_00618b70(0x38);
          uStack_4 = 3;
          if (pvStack_18 == (void *)0x0) {
            pvVar3 = (void *)0x0;
          }
          else {
            pvVar3 = FUN_0041ca70(pvStack_18,
                                  *(undefined4 *)(*(int *)((int)param_1 + 0x60) + iVar5 * 4));
          }
          uStack_4 = 0xffffffff;
          (**(code **)(**(int **)((int)param_1 + 0xc) + 0x2c))(pvVar3);
          (**(code **)(**(int **)((int)param_1 + 0x10) + 0x10))(iVar5,pvVar3);
        }
        else {
          (**(code **)(*piVar2 + 8))(*(undefined4 *)(*(int *)((int)param_1 + 0x60) + iVar5 * 4));
        }
        iVar5 = iVar5 + 1;
        iStack_28 = iStack_28 + 0xc;
        psStack_30 = psStack_30 + 1;
        hResData = uStack_20;
      } while (iVar5 < *(int *)((int)param_1 + 0x48));
    }
    if (*local_2c != 0) {
      uStack_20 = (HGLOBAL)CONCAT22(uStack_12,*local_2c);
      FUN_00403f00(param_1,(ushort *)&uStack_20,0);
    }
    if (local_2c[1] != 0) {
      uStack_20 = (HGLOBAL)CONCAT22(local_2c[2],local_2c[1]);
      FUN_00403f70(param_1,(ushort *)&uStack_20,0);
    }
    FreeResource(hResData);
  }
  iVar5 = 0;
  if (0 < *(int *)((int)param_1 + 0x48)) {
    iVar6 = 0;
    do {
      iVar4 = thunk_FUN_005f5060(iVar6 + *(int *)((int)param_1 + 0x58));
      iVar5 = iVar5 + 1;
      iVar6 = iVar6 + 0xc;
      *(int *)(*(int *)((int)param_1 + 0x5c) + -4 + iVar5 * 4) = iVar4;
    } while (iVar5 < *(int *)((int)param_1 + 0x48));
  }
  ExceptionList = pvStack_c;
  return;
}

