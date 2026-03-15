
int __cdecl FUN_0061e470(int *param_1,char *param_2,undefined4 *param_3)

{
  WCHAR WVar1;
  uint uVar2;
  short *psVar3;
  int *piVar4;
  undefined4 uVar5;
  WCHAR *pWVar6;
  LPSTR pCVar7;
  char cVar8;
  LPSTR pCVar9;
  LPSTR pCVar10;
  char *pcVar11;
  int iVar12;
  ulonglong uVar13;
  undefined8 uVar14;
  longlong lVar15;
  uint uVar16;
  uint local_24c;
  WCHAR *local_248;
  int local_244;
  int local_240;
  char local_23a;
  char local_239;
  int local_238;
  int local_234;
  int local_230;
  uint local_22c;
  int local_228;
  int local_224;
  int local_220;
  uint local_21c;
  undefined4 local_218;
  CHAR local_214 [4];
  undefined4 local_210;
  undefined4 local_20c;
  uint local_204;
  undefined1 local_200 [511];
  CHAR local_1;
  
  local_220 = 0;
  pCVar10 = (LPSTR)0x0;
  local_240 = 0;
  cVar8 = *param_2;
  param_2 = param_2 + 1;
  local_21c = CONCAT31(local_21c._1_3_,cVar8);
  do {
    if ((cVar8 == '\0') || (local_240 < 0)) {
      return local_240;
    }
    if ((cVar8 < ' ') || ('x' < cVar8)) {
      uVar2 = 0;
    }
    else {
      uVar2 = *(byte *)((int)&PTR_LAB_0066e720 + (int)cVar8) & 0xf;
    }
    local_220 = (int)(char)(&DAT_0066e740)[uVar2 * 8 + local_220] >> 4;
    switch(local_220) {
    case 0:
switchD_0061e4ed_caseD_0:
      local_230 = 0;
      if ((PTR_DAT_006af1a0[(local_21c & 0xff) * 2 + 1] & 0x80) != 0) {
        FUN_0061ee00((int)cVar8,param_1,&local_240);
        cVar8 = *param_2;
        param_2 = param_2 + 1;
      }
      FUN_0061ee00((int)cVar8,param_1,&local_240);
      break;
    case 1:
      local_218 = 0;
      local_228 = 0;
      local_234 = 0;
      local_238 = 0;
      local_24c = 0;
      local_244 = -1;
      local_230 = 0;
      break;
    case 2:
      switch(cVar8) {
      case ' ':
        local_24c = local_24c | 2;
        break;
      case '#':
        local_24c = local_24c | 0x80;
        break;
      case '+':
        local_24c = local_24c | 1;
        break;
      case '-':
        local_24c = local_24c | 4;
        break;
      case '0':
        local_24c = local_24c | 8;
      }
      break;
    case 3:
      if (cVar8 == '*') {
        local_234 = FUN_0061eed0((int *)&param_3);
        if (local_234 < 0) {
          local_24c = local_24c | 4;
          local_234 = -local_234;
        }
      }
      else {
        local_234 = cVar8 + -0x30 + local_234 * 10;
      }
      break;
    case 4:
      local_244 = 0;
      break;
    case 5:
      if (cVar8 == '*') {
        local_244 = FUN_0061eed0((int *)&param_3);
        if (local_244 < 0) {
          local_244 = -1;
        }
      }
      else {
        local_244 = cVar8 + -0x30 + local_244 * 10;
      }
      break;
    case 6:
      switch(cVar8) {
      case 'I':
        if ((*param_2 != '6') || (param_2[1] != '4')) {
          local_220 = 0;
          goto switchD_0061e4ed_caseD_0;
        }
        param_2 = param_2 + 2;
        local_24c = local_24c | 0x8000;
        break;
      case 'h':
        local_24c = local_24c | 0x20;
        break;
      case 'l':
        local_24c = local_24c | 0x10;
        break;
      case 'w':
        local_24c = local_24c | 0x800;
      }
      break;
    case 7:
      switch(cVar8) {
      case 'C':
        if ((local_24c & 0x830) == 0) {
          local_24c = local_24c | 0x800;
        }
      case 'c':
        if ((local_24c & 0x810) == 0) {
          uVar5 = FUN_0061eed0((int *)&param_3);
          local_200[0] = (char)uVar5;
          pCVar10 = (LPSTR)0x1;
        }
        else {
          uVar5 = FUN_0061ef10(&param_3);
          pCVar10 = FUN_00624b10(local_200,(WCHAR)uVar5);
          if ((int)pCVar10 < 0) {
            local_248 = (WCHAR *)local_200;
            local_228 = 1;
            break;
          }
        }
        local_248 = (WCHAR *)local_200;
        break;
      case 'E':
      case 'G':
        local_218 = 1;
        cVar8 = cVar8 + ' ';
      case 'e':
      case 'f':
      case 'g':
        local_248 = (WCHAR *)local_200;
        if (local_244 < 0) {
          local_244 = 6;
        }
        else if ((local_244 == 0) && (cVar8 == 'g')) {
          local_244 = 1;
        }
        local_210 = *param_3;
        local_20c = param_3[1];
        param_3 = param_3 + 2;
        (*(code *)PTR_FUN_006af4e0)(&local_210,local_200,(int)cVar8,local_244,local_218);
        if (((local_24c & 0x80) != 0) && (local_244 == 0)) {
          (*(code *)PTR_FUN_006af4ec)(local_200);
        }
        if ((cVar8 == 'g') && ((local_24c & 0x80) == 0)) {
          (*(code *)PTR_FUN_006af4e4)(local_200);
        }
        uVar2 = local_24c | 0x40;
        if (local_200[0] == '-') {
          local_248 = (WCHAR *)(local_200 + 1);
          uVar2 = local_24c | 0x140;
        }
        local_24c = uVar2;
        uVar2 = 0xffffffff;
        pWVar6 = local_248;
        do {
          if (uVar2 == 0) break;
          uVar2 = uVar2 - 1;
          WVar1 = *pWVar6;
          pWVar6 = (WCHAR *)((int)pWVar6 + 1);
        } while ((char)WVar1 != '\0');
        pCVar10 = (LPSTR)(~uVar2 - 1);
        break;
      case 'S':
        if ((local_24c & 0x830) == 0) {
          local_24c = local_24c | 0x800;
        }
      case 's':
        iVar12 = 0x7fffffff;
        if (local_244 != -1) {
          iVar12 = local_244;
        }
        local_248 = (WCHAR *)FUN_0061eed0((int *)&param_3);
        if ((local_24c & 0x810) == 0) {
          pWVar6 = local_248;
          if (local_248 == (WCHAR *)0x0) {
            pWVar6 = (WCHAR *)PTR_DAT_006af198;
            local_248 = (WCHAR *)PTR_DAT_006af198;
          }
          for (; (iVar12 != 0 && (iVar12 = iVar12 + -1, (char)*pWVar6 != '\0'));
              pWVar6 = (WCHAR *)((int)pWVar6 + 1)) {
          }
          pCVar10 = (LPSTR)((int)pWVar6 - (int)local_248);
        }
        else {
          if (local_248 == (WCHAR *)0x0) {
            local_248 = (WCHAR *)PTR_DAT_006af19c;
          }
          local_230 = 1;
          for (pWVar6 = local_248; (iVar12 != 0 && (iVar12 = iVar12 + -1, *pWVar6 != L'\0'));
              pWVar6 = pWVar6 + 1) {
          }
          pCVar10 = (LPSTR)((int)pWVar6 - (int)local_248 >> 1);
        }
        break;
      case 'X':
        goto switchD_0061e701_caseD_58;
      case 'Z':
        psVar3 = (short *)FUN_0061eed0((int *)&param_3);
        if ((psVar3 == (short *)0x0) ||
           (local_248 = *(WCHAR **)(psVar3 + 2), local_248 == (WCHAR *)0x0)) {
          uVar2 = 0xffffffff;
          local_248 = (WCHAR *)PTR_DAT_006af198;
          pcVar11 = PTR_DAT_006af198;
          do {
            if (uVar2 == 0) break;
            uVar2 = uVar2 - 1;
            cVar8 = *pcVar11;
            pcVar11 = pcVar11 + 1;
          } while (cVar8 != '\0');
          pCVar10 = (LPSTR)(~uVar2 - 1);
        }
        else if ((local_24c & 0x800) == 0) {
          pCVar10 = (LPSTR)(int)*psVar3;
          local_230 = 0;
        }
        else {
          local_230 = 1;
          pCVar10 = (LPSTR)((uint)(int)*psVar3 >> 1);
        }
        break;
      case 'd':
      case 'i':
        local_22c = 10;
        local_24c = local_24c | 0x40;
        goto LAB_0061ea37;
      case 'n':
        piVar4 = (int *)FUN_0061eed0((int *)&param_3);
        if ((local_24c & 0x20) == 0) {
          local_228 = 1;
          *piVar4 = local_240;
        }
        else {
          local_228 = 1;
          *(undefined2 *)piVar4 = (undefined2)local_240;
        }
        break;
      case 'o':
        local_22c = 8;
        if ((local_24c & 0x80) != 0) {
          local_24c = local_24c | 0x200;
        }
        goto LAB_0061ea37;
      case 'p':
        local_244 = 8;
switchD_0061e701_caseD_58:
        local_224 = 7;
LAB_0061e9f2:
        local_22c = 0x10;
        if ((local_24c & 0x80) != 0) {
          local_23a = '0';
          local_239 = (char)local_224 + 'Q';
          local_238 = 2;
        }
        goto LAB_0061ea37;
      case 'u':
        local_22c = 10;
LAB_0061ea37:
        if ((local_24c & 0x8000) == 0) {
          if ((local_24c & 0x20) == 0) {
            if ((local_24c & 0x40) == 0) {
              uVar2 = FUN_0061eed0((int *)&param_3);
              uVar13 = (ulonglong)uVar2;
            }
            else {
              iVar12 = FUN_0061eed0((int *)&param_3);
              uVar13 = (ulonglong)iVar12;
            }
          }
          else if ((local_24c & 0x40) == 0) {
            uVar2 = FUN_0061eed0((int *)&param_3);
            uVar13 = (ulonglong)uVar2 & 0xffffffff0000ffff;
          }
          else {
            uVar5 = FUN_0061eed0((int *)&param_3);
            uVar13 = (ulonglong)(int)(short)uVar5;
          }
        }
        else {
          uVar13 = FUN_0061eef0((int *)&param_3);
        }
        iVar12 = (int)(uVar13 >> 0x20);
        if ((((local_24c & 0x40) != 0) && (iVar12 == 0 || (longlong)uVar13 < 0)) &&
           ((longlong)uVar13 < 0)) {
          local_24c = local_24c | 0x100;
          uVar13 = CONCAT44(-(iVar12 + (uint)((int)uVar13 != 0)),-(int)uVar13);
        }
        iVar12 = (int)(uVar13 >> 0x20);
        if ((local_24c & 0x8000) == 0) {
          iVar12 = 0;
        }
        lVar15 = CONCAT44(iVar12,(int)uVar13);
        if (local_244 < 0) {
          local_244 = 1;
        }
        else {
          local_24c = local_24c & 0xfffffff7;
        }
        if ((int)uVar13 == 0 && iVar12 == 0) {
          local_238 = 0;
        }
        pWVar6 = (WCHAR *)&local_1;
        iVar12 = local_244;
        while ((uVar2 = local_22c, local_244 = iVar12 + -1, 0 < iVar12 || (lVar15 != 0))) {
          local_204 = (int)local_22c >> 0x1f;
          uVar16 = (uint)((ulonglong)lVar15 >> 0x20);
          uVar14 = __aullrem((uint)lVar15,uVar16,local_22c,local_204);
          iVar12 = (int)uVar14 + 0x30;
          lVar15 = __aulldiv((uint)lVar15,uVar16,uVar2,local_204);
          if (0x39 < iVar12) {
            iVar12 = iVar12 + local_224;
          }
          *(char *)pWVar6 = (char)iVar12;
          pWVar6 = (WCHAR *)((int)pWVar6 + -1);
          iVar12 = local_244;
        }
        pCVar10 = &local_1 + -(int)pWVar6;
        local_248 = (WCHAR *)((int)pWVar6 + 1);
        if (((local_24c & 0x200) != 0) && ((*(char *)local_248 != '0' || (pCVar10 == (LPSTR)0x0))))
        {
          pCVar10 = &stack0x00000000 + -(int)pWVar6;
          *(undefined1 *)pWVar6 = 0x30;
          local_248 = pWVar6;
        }
        break;
      case 'x':
        local_224 = 0x27;
        goto LAB_0061e9f2;
      }
      if (local_228 == 0) {
        if ((local_24c & 0x40) != 0) {
          if ((local_24c & 0x100) == 0) {
            if ((local_24c & 1) == 0) {
              if ((local_24c & 2) == 0) goto LAB_0061ebcf;
              local_23a = ' ';
            }
            else {
              local_23a = '+';
            }
          }
          else {
            local_23a = '-';
          }
          local_238 = 1;
        }
LAB_0061ebcf:
        iVar12 = (local_234 - (int)pCVar10) - local_238;
        if ((local_24c & 0xc) == 0) {
          FUN_0061ee50(0x20,iVar12,param_1,&local_240);
        }
        FUN_0061ee90(&local_23a,local_238,param_1,&local_240);
        if (((local_24c & 8) != 0) && ((local_24c & 4) == 0)) {
          FUN_0061ee50(0x30,iVar12,param_1,&local_240);
        }
        if ((local_230 == 0) || (pWVar6 = local_248, pCVar9 = pCVar10, (int)pCVar10 < 1)) {
          FUN_0061ee90((char *)local_248,(int)pCVar10,param_1,&local_240);
        }
        else {
          do {
            pCVar9 = pCVar9 + -1;
            pCVar7 = FUN_00624b10(local_214,*pWVar6);
            if ((int)pCVar7 < 1) break;
            FUN_0061ee90(local_214,(int)pCVar7,param_1,&local_240);
            pWVar6 = pWVar6 + 1;
          } while (pCVar9 != (LPSTR)0x0);
        }
        if ((local_24c & 4) != 0) {
          FUN_0061ee50(0x20,iVar12,param_1,&local_240);
        }
      }
    }
    cVar8 = *param_2;
    param_2 = param_2 + 1;
    local_21c = CONCAT31(local_21c._1_3_,cVar8);
  } while( true );
}

