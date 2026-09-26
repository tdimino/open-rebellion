
void __thiscall FUN_00452630(void *this,HMODULE param_1,void *param_2)

{
  ushort uVar1;
  ushort uVar2;
  ushort uVar3;
  ushort uVar4;
  ushort uVar5;
  ushort uVar6;
  ushort uVar7;
  ushort uVar8;
  undefined4 *puVar9;
  HRSRC hResInfo;
  HGLOBAL hResData;
  ushort *puVar10;
  void *pvVar11;
  undefined4 uVar12;
  HDC hdc;
  HGDIOBJ h;
  uint *this_00;
  HDC hdc_00;
  ushort *puVar13;
  undefined2 uVar14;
  uint *this_01;
  uint uVar15;
  uint uVar16;
  HGDIOBJ pvStack_14;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0062fd4c;
  pvStack_c = ExceptionList;
  hdc_00 = (HDC)0x0;
  ExceptionList = &pvStack_c;
  if (((param_1 != (HMODULE)0x0) && (ExceptionList = &pvStack_c, (short)param_2 != 0)) &&
     (ExceptionList = &pvStack_c,
     hResInfo = FindResourceA(param_1,(LPCSTR)((uint)param_2 & 0xffff),(LPCSTR)0xa),
     hResInfo != (HRSRC)0x0)) {
    hResData = LoadResource(param_1,hResInfo);
    puVar10 = LockResource(hResData);
    uVar1 = *puVar10;
    uVar2 = puVar10[1];
    if ((uVar1 < (ushort)*(undefined4 *)((int)this + 0x38)) ||
       (uVar2 < *(ushort *)((int)this + 0x3c))) {
      SetWindowPos(*(HWND *)((int)this + 0x18),(HWND)0x0,0,0,(uint)uVar1,(uint)uVar2,6);
      puVar9 = *(undefined4 **)((int)this + 0xa4);
      *(undefined4 *)((int)this + 0x30) = *(undefined4 *)((int)this + 0x38);
      *(undefined4 *)((int)this + 0x34) = *(undefined4 *)((int)this + 0x3c);
      if (puVar9 != (undefined4 *)0x0) {
        FUN_005fbfa0(puVar9);
        FUN_00618b60((undefined *)puVar9);
      }
      if (*(undefined **)((int)this + 0x140) != (undefined *)0x0) {
        FUN_00618b60(*(undefined **)((int)this + 0x140));
      }
      param_2 = (void *)FUN_00618b70(0x28);
      uStack_4 = 0;
      if (param_2 == (void *)0x0) {
        pvVar11 = (void *)0x0;
      }
      else {
        pvVar11 = FUN_005fbda0(param_2,*(int *)((int)this + 0x38),*(int *)((int)this + 0x3c),
                               *(uint **)(*(int *)((int)this + 0x11c) + 0x45c));
      }
      uStack_4 = 0xffffffff;
      *(void **)((int)this + 0xa4) = pvVar11;
      uVar12 = FUN_005fddd0(pvVar11,&LAB_004ac550);
      *(undefined4 *)((int)this + 0x140) = uVar12;
    }
    hdc = GetDC(*(HWND *)((int)this + 0x18));
    param_2 = (void *)FUN_00618b70(0x28);
    uStack_4 = 1;
    if (param_2 == (void *)0x0) {
      pvVar11 = (void *)0x0;
    }
    else {
      pvVar11 = FUN_005fbea0(param_2,(uint)uVar1,(uint)uVar2,*(uint **)((int)this + 0xa4),hdc);
    }
    uStack_4 = 0xffffffff;
    *(void **)((int)this + 0x148) = pvVar11;
    hdc_00 = CreateCompatibleDC(hdc);
    h = (HGDIOBJ)FUN_005fc130(*(int *)((int)this + 0x148));
    pvStack_14 = SelectObject(hdc_00,h);
    ReleaseDC(*(HWND *)((int)this + 0x18),hdc);
    uVar12 = FUN_006037f0((uint)puVar10[2]);
    uVar2 = puVar10[3];
    puVar10 = puVar10 + 4;
    uVar3 = *puVar10;
    this_01 = this;
    while (uVar3 != 0) {
      if (uVar3 != 0) {
        uVar14 = (undefined2)((uint)this_01 >> 0x10);
        if (uVar3 < 3) {
          uVar3 = puVar10[1];
          uVar4 = puVar10[2];
          puVar13 = puVar10 + 3;
          this_01 = (uint *)CONCAT22(uVar14,*puVar13);
          puVar10 = puVar10 + 3;
          if (*puVar13 != 0) {
            pvVar11 = (void *)FUN_00618b70(0x28);
            uStack_4 = 2;
            if (pvVar11 == (void *)0x0) {
              this_01 = (uint *)0x0;
            }
            else {
              this_01 = FUN_005fbd20(pvVar11,uVar12,(uint)this_01,10);
            }
            uStack_4 = 0xffffffff;
            if (this_01 != (uint *)0x0) {
              FUN_005fd0f0(this_01,*(uint **)((int)this + 0x148),(uint)uVar3,(uint)uVar4);
              FUN_005fbfa0(this_01);
              this_00 = this_01;
LAB_004529fc:
              FUN_00618b60((undefined *)this_00);
            }
          }
        }
        else if (uVar3 == 3) {
          uVar3 = puVar10[1];
          uVar4 = puVar10[2];
          puVar13 = puVar10 + 3;
          uVar5 = puVar10[4];
          uVar6 = puVar10[5];
          uVar7 = puVar10[6];
          puVar10 = puVar10 + 7;
          uVar8 = *puVar10;
          this_01 = (uint *)CONCAT22(uVar14,uVar8);
          if (*puVar13 != 0) {
            param_2 = (void *)CONCAT22(uVar2,*puVar13);
            pvVar11 = (void *)FUN_00618b70(0x34);
            uStack_4 = 3;
            if (pvVar11 == (void *)0x0) {
              this_00 = (uint *)0x0;
            }
            else {
              this_00 = FUN_00601700(pvVar11,&param_2,0,0,0,0,0,0);
            }
            uStack_4 = 0xffffffff;
            if (this_00 != (uint *)0x0) {
              this_01 = (uint *)(((uVar8 & 0xff | 0x200) << 8 | uVar7 & 0xff) << 8 | uVar6 & 0xff);
              FUN_00601c90(this_00,(int)this_01,(HWND)0x0);
              FUN_00601c60(this_00,(uint)uVar5);
              if (uVar4 == 1) {
                *this_00 = (uint)uVar1;
                this_00[0xb] = 1;
                FUN_00403e90(this_00,1);
                uVar16 = 1;
                uVar15 = 0;
              }
              else {
                uVar16 = (uint)uVar4;
                uVar15 = (uint)uVar3;
              }
              FUN_00601b30(this_00,uVar15,uVar16);
              FUN_00601ce0(this_00,hdc_00);
              FUN_00601990((int)this_00);
              goto LAB_004529fc;
            }
          }
        }
      }
      puVar13 = puVar10 + 1;
      puVar10 = puVar10 + 1;
      uVar3 = *puVar13;
    }
    FreeResource(hResData);
  }
  if (hdc_00 != (HDC)0x0) {
    SelectObject(hdc_00,pvStack_14);
    DeleteObject(hdc_00);
  }
  ExceptionList = pvStack_c;
  return;
}

